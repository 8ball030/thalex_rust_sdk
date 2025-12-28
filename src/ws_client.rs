use serde::{Deserialize, de::DeserializeOwned};

use tokio::{
    sync::oneshot,
    time::{Duration, Instant, MissedTickBehavior, interval, sleep},
};

use futures_util::{SinkExt, StreamExt};
use log::{error, info, warn};
use serde_json::Value;
use std::collections::HashMap;
use std::env::var;
use std::sync::{
    Arc,
    atomic::{AtomicU64, Ordering},
};
use thiserror::Error;
use tokio::sync::{Mutex, mpsc, watch};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

use crate::{
    auth_utils::make_auth_token,
    models::{Instrument, RpcErrorResponse, RpcResponse},
    types::{
        Error, ExternalEvent, InternalCommand, LoginState, RequestScope, ResponseSender, WsStream,
    },
    utils::round_to_ticks,
};

#[derive(Deserialize)]
#[serde(untagged)]
enum SubscribeResponse {
    Ok { id: u64, result: Vec<String> },
    Err { id: u64, error: RpcErrorResponse },
}

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("RPC error: {0:?}")]
    Rpc(RpcErrorResponse),

    #[error("transport error")]
    Transport(#[from] Box<dyn std::error::Error + Send + Sync>),

    #[error("JSON parse error")]
    Parse(#[from] serde_json::Error),

    #[error("oneshot receive error")]
    Recv(#[from] oneshot::error::RecvError),
}

use crate::channels::subscriptions::Subscriptions;
use crate::rpc::Rpc;

const URL: &str = "wss://thalex.com/ws/api/v2";
const PING_INTERVAL: Duration = Duration::from_secs(5);
const READ_TIMEOUT: Duration = Duration::from_secs(10);

pub struct WsClient {
    write_tx: mpsc::UnboundedSender<InternalCommand>,
    pending_requests: Arc<Mutex<HashMap<u64, ResponseSender>>>,
    pub public_subscriptions: Arc<Mutex<HashMap<String, mpsc::UnboundedSender<String>>>>,
    pub private_subscriptions: Arc<Mutex<HashMap<String, mpsc::UnboundedSender<String>>>>,
    next_id: Arc<AtomicU64>,
    shutdown_tx: watch::Sender<bool>,
    instruments_cache: Arc<Mutex<HashMap<String, Instrument>>>,
    login_state: LoginState,
    connection_state_rx: watch::Receiver<ExternalEvent>,
}

impl WsClient {
    pub fn subscriptions(&self) -> Subscriptions {
        Subscriptions { client: self }
    }

    pub fn rpc(&self) -> Rpc {
        Rpc { client: self }
    }

    pub async fn from_env() -> Result<Self, Error> {
        let key_path = var("THALEX_PRIVATE_KEY_PATH").unwrap();
        let key_id = var("THALEX_KEY_ID").unwrap();
        let account_id = var("THALEX_ACCOUNT_ID").unwrap();
        let client = WsClient::new(URL, key_id, account_id, key_path).await?;
        info!("WsClient created from environment variables Logging in...");
        client.login().await.expect("Login failed");
        Ok(client)
    }

    pub async fn new_public() -> Result<Self, Error> {
        WsClient::new(URL, "".to_string(), "".to_string(), "".to_string()).await
    }

    pub async fn new(
        url: impl Into<String>,
        key_id: String,
        account_id: String,
        key_path: String,
    ) -> Result<Self, Error> {
        let url = url.into();

        let (cmd_tx, cmd_rx) = mpsc::unbounded_channel::<InternalCommand>();
        let (shutdown_tx, shutdown_rx) = watch::channel(false);

        let pending_requests = Arc::new(Mutex::new(HashMap::new()));
        let public_subscriptions = Arc::new(Mutex::new(HashMap::new()));
        let private_subscriptions = Arc::new(Mutex::new(HashMap::new()));
        let next_id = Arc::new(AtomicU64::new(1));

        let (connection_state_tx, connection_state_rx) =
            watch::channel(ExternalEvent::Disconnected);

        let login_state = LoginState {
            key_id,
            account_id,
            key_path,
        };

        let _ = connection_state_tx.send(ExternalEvent::Disconnected);

        let client = WsClient {
            write_tx: cmd_tx.clone(),
            pending_requests: pending_requests.clone(),
            public_subscriptions: public_subscriptions.clone(),
            private_subscriptions: private_subscriptions.clone(),
            next_id: next_id.clone(),
            shutdown_tx: shutdown_tx.clone(),
            instruments_cache: Arc::new(Mutex::new(HashMap::new())),
            login_state,
            connection_state_rx,
        };

        tokio::spawn(connection_supervisor(
            url,
            cmd_rx,
            shutdown_rx,
            pending_requests,
            public_subscriptions,
            private_subscriptions,
            connection_state_tx,
        ));
        // client.cache_instruments().await?;
        Ok(client)
    }

    async fn cache_instruments(&self) -> Result<(), Error> {
        let instruments = self.get_instruments().await.unwrap();
        let mut cache = self.instruments_cache.lock().await;
        cache.clear();
        for instrument in &instruments {
            cache.insert(
                instrument.instrument_name.clone().unwrap(),
                instrument.clone(),
            );
        }
        Ok(())
    }

    pub async fn round_price_to_ticks(
        &self,
        price: f64,
        instrument_name: &str,
    ) -> Result<f64, Error> {
        let instrument = self
            .instruments_cache
            .lock()
            .await
            .get(instrument_name)
            .cloned();
        // refresh cache if not found
        if let Some(instr) = instrument {
            Ok(round_to_ticks(price, instr.tick_size.unwrap()))
        } else {
            self.cache_instruments().await?;
            let cache = self.instruments_cache.lock().await;
            if let Some(instr) = cache.get(instrument_name).cloned() {
                Ok(round_to_ticks(price, instr.tick_size.unwrap()))
            } else {
                Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    format!("Instrument not found: {instrument_name}"),
                )))
            }
        }
    }

    async fn get_instruments(&self) -> Result<Vec<Instrument>, RpcErrorResponse> {
        self.rpc().market_data().instruments().await
    }

    pub async fn send_rpc<T>(
        &self,
        method: &str,
        params: serde_json::Value,
    ) -> Result<T, ClientError>
    where
        T: serde::de::DeserializeOwned,
    {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);

        let (tx, rx) = oneshot::channel::<String>();
        {
            let mut pending = self.pending_requests.lock().await;
            pending.insert(id, tx);
        }

        let request = serde_json::json!({
            "jsonrpc": "2.0",
            "id": id,
            "method": method,
            "params": params
        });

        let text = request.to_string();

        if let Err(e) = self
            .write_tx
            .send(InternalCommand::Send(Message::Text(text.into())))
        {
            let mut pending = self.pending_requests.lock().await;
            pending.remove(&id);
            return Err(ClientError::Transport(Box::new(e)));
        }

        let response = rx.await?;
        println!("RPC Response: {response}");

        let envelope: T = serde_json::from_str(&response)?;
        Ok(envelope)
    }

    pub async fn shutdown(&self, reason: &'static str) -> Result<(), Error> {
        info!("Shutdown requested: {reason}");
        let _ = self.shutdown_tx.send(true);
        let _ = self.write_tx.send(InternalCommand::Close);
        Ok(())
    }

    pub async fn subscribe_channel<P, F>(
        &self,
        scope: RequestScope,
        channel: String,
        mut callback: F,
    ) -> Result<String, ClientError>
    where
        P: DeserializeOwned + Send + 'static,
        F: FnMut(P) + Send + 'static,
    {
        let sub_result: SubscribeResponse = self
            .send_rpc(
                &format!("{scope}/subscribe"),
                serde_json::json!({
                    "channels": [channel.clone()]
                }),
            )
            .await?;
        match sub_result {
            SubscribeResponse::Ok {
                id: _id,
                result: _result,
            } => {
                let (tx, mut rx) = mpsc::unbounded_channel::<String>();

                {
                    match scope {
                        RequestScope::Public => {
                            let mut subs = self.public_subscriptions.lock().await;
                            subs.insert(channel.clone(), tx);
                            info!("Subscribed to public channel: {channel}");
                        }
                        RequestScope::Private => {
                            let mut subs = self.private_subscriptions.lock().await;
                            subs.insert(channel.clone(), tx);
                            info!("Subscribed to private channel: {channel}");
                        }
                    }
                }

                tokio::spawn(async move {
                    while let Some(msg) = rx.recv().await {
                        let parsed: P = match serde_json::from_str(&msg) {
                            Ok(m) => m,
                            Err(e) => {
                                warn!("Failed to parse channel message: {e}; raw: {msg}");
                                continue;
                            }
                        };

                        callback(parsed);
                    }
                });
                Ok(channel)
            }
            SubscribeResponse::Err { error, id: _id } => {
                warn!("Subscription error: {error:?}");
                Err(ClientError::Rpc(error))
            }
        }
    }

    pub async fn unsubscribe(&self, channel: &str) -> Result<(), Error> {
        let channel = channel.to_string();

        {
            let mut subs = self.public_subscriptions.lock().await;
            if subs.remove(&channel).is_some() {
                let _: RpcResponse = self
                    .send_rpc(
                        "public/unsubscribe",
                        serde_json::json!({
                            "channels": [channel.clone()]
                        }),
                    )
                    .await?;
                info!("Unsubscribed from public channel: {channel}");
                return Ok(());
            }
        }
        {
            let mut subs = self.private_subscriptions.lock().await;
            if subs.remove(&channel).is_some() {
                let _: RpcResponse = self
                    .send_rpc(
                        "private/unsubscribe",
                        serde_json::json!({
                            "channels": [channel.clone()]
                        }),
                    )
                    .await?;
                info!("Unsubscribed from private channel: {channel}");
                return Ok(());
            }
        }
        warn!("No active subscription found for channel: {channel}");
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("No active subscription for channel: {channel}"),
        )))
    }

    pub async fn login(&self) -> Result<(), Error> {
        let private_key_pem = tokio::fs::read_to_string(&self.login_state.key_path).await?;
        let token = make_auth_token(&self.login_state.key_id, private_key_pem)?;
        let result: Value = self
            .send_rpc(
                "public/login",
                serde_json::json!({
                    "token": token,
                    "account": &self.login_state.account_id
                }),
            )
            .await?;
        info!("Sent login message, received response: {result:?}");
        if let Some(error) = result.get("error") {
            Err(Box::new(std::io::Error::other(format!(
                "Login error: {error:?}"
            ))))
        } else {
            info!("Login successful");
            Ok(())
        }
    }

    pub async fn set_cancel_on_disconnect(&self) -> Result<(), Error> {
        let result: RpcResponse = self
            .send_rpc(
                "private/set_cancel_on_disconnect",
                serde_json::json!({ "timeout_secs": 6}),
            )
            .await?;
        info!("Set cancel_on_disconnect result: {result:?}");
        Ok(())
    }

    pub async fn run_till_event(&self) -> ExternalEvent {
        let mut rx = self.connection_state_rx.clone();
        let current_state = *rx.borrow();
        loop {
            if rx.changed().await.is_ok() {
                let event = *rx.borrow();
                if current_state != event {
                    return event;
                }
            }
        }
    }

    pub async fn is_connected(&self) -> bool {
        *self.connection_state_rx.borrow() == ExternalEvent::Connected
    }

    pub async fn resubscribe_all(&self) -> Result<(), Error> {
        let public_channels: Vec<String> = {
            let subs = self.public_subscriptions.lock().await;
            subs.keys().cloned().collect()
        };
        for channel in public_channels {
            let _: RpcResponse = self
                .send_rpc(
                    "public/subscribe",
                    serde_json::json!({
                        "channels": [channel.clone()]
                    }),
                )
                .await?;
            info!("Re-subscribed to channel: {channel}");
        }
        let private_channels: Vec<String> = {
            let subs = self.private_subscriptions.lock().await;
            subs.keys().cloned().collect()
        };
        for channel in private_channels {
            let _: RpcResponse = self
                .send_rpc(
                    "private/subscribe",
                    serde_json::json!({
                        "channels": [channel.clone()]
                    }),
                )
                .await?;
            info!("Re-subscribed to channel: {channel}");
        }
        Ok(())
    }
    pub async fn wait_for_connection(&self) {
        loop {
            if self.is_connected().await {
                break;
            }
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }
}

async fn connection_supervisor(
    url: String,
    mut cmd_rx: mpsc::UnboundedReceiver<InternalCommand>,
    mut shutdown_rx: watch::Receiver<bool>,
    pending_requests: Arc<Mutex<HashMap<u64, ResponseSender>>>,
    public_subscriptions: Arc<Mutex<HashMap<String, mpsc::UnboundedSender<String>>>>,
    private_subscriptions: Arc<Mutex<HashMap<String, mpsc::UnboundedSender<String>>>>,
    connection_state_tx: watch::Sender<ExternalEvent>,
) {
    info!("Connection supervisor started for {url}");

    loop {
        if *shutdown_rx.borrow() {
            info!("Supervisor sees shutdown for {url}");
            break;
        }

        match connect_async(&url).await {
            Ok((ws_stream, _)) => {
                let current = *connection_state_tx.borrow();
                if current != ExternalEvent::Connected {
                    info!("WebSocket connected to {url}");
                    connection_state_tx.send(ExternalEvent::Connected).ok();
                }
                let result = run_single_connection(
                    &url,
                    ws_stream,
                    &mut cmd_rx,
                    &mut shutdown_rx,
                    &pending_requests,
                    &public_subscriptions,
                    &private_subscriptions,
                )
                .await;

                if let Err(e) = result {
                    error!("Connection error on {url}: {e}");
                }

                // Fail all pending RPCs on this connection.
                let mut pending = pending_requests.lock().await;
                for (_, tx) in pending.drain() {
                    let _ = tx.send(r#"{"error":"connection closed"}"#.to_string());
                }

                if *shutdown_rx.borrow() {
                    info!("Shutdown after connection end for {url}");
                    break;
                }

                if cmd_rx.is_closed() {
                    info!("Command channel closed for {url}, stopping supervisor");
                    break;
                }

                info!("Reconnecting to {url} after backoff");
                tokio::time::sleep(std::time::Duration::from_secs(3)).await;
            }
            Err(e) => {
                error!("Failed to connect to {url}: {e}");
                connection_state_tx
                    .send(ExternalEvent::Disconnected)
                    .unwrap();
                if *shutdown_rx.borrow() || cmd_rx.is_closed() {
                    break;
                }
                tokio::time::sleep(std::time::Duration::from_secs(3)).await;
            }
        }
    }

    info!("Connection supervisor exited for {url}");
}

async fn run_single_connection(
    url: &str,
    mut ws: WsStream,
    cmd_rx: &mut mpsc::UnboundedReceiver<InternalCommand>,
    shutdown_rx: &mut watch::Receiver<bool>,
    pending_requests: &Arc<Mutex<HashMap<u64, ResponseSender>>>,
    public_subscriptions: &Arc<Mutex<HashMap<String, mpsc::UnboundedSender<String>>>>,
    private_subscriptions: &Arc<Mutex<HashMap<String, mpsc::UnboundedSender<String>>>>,
) -> Result<(), Error> {
    // Set up ping interval
    let mut ping_interval = interval(PING_INTERVAL);
    ping_interval.set_missed_tick_behavior(MissedTickBehavior::Delay);

    let read_deadline = sleep(READ_TIMEOUT);
    tokio::pin!(read_deadline);

    loop {
        tokio::select! {
            _ = ping_interval.tick() => {
                if let Err(e) = ws.send(Message::Ping(Vec::new().into())).await {
                    warn!("Failed to send ping for {url}: {e}");
                    return Err(Box::new(e));
                }
            }

            _ = shutdown_rx.changed() => {
                if *shutdown_rx.borrow() {
                    info!("Shutdown requested for {url}");
                    let _ = ws.close(None).await;
                    return Ok(());
                }
            }

            maybe_cmd = cmd_rx.recv() => {
                match maybe_cmd {
                    Some(InternalCommand::Send(msg)) => {
                        ws.send(msg).await?;
                    }
                    Some(InternalCommand::Close) => {
                        info!("Close command received for {url}");
                        let _ = ws.close(None).await;
                        return Ok(());
                    }
                    None => {
                        info!("Command channel closed for {url}");
                        let _ = ws.close(None).await;
                        return Ok(());
                    }
                }
            }

            msg = ws.next() => {
                read_deadline.as_mut().reset(Instant::now() + READ_TIMEOUT);

                match msg {
                    Some(Ok(Message::Text(text))) => {
                        handle_incoming(
                            text.to_string(),
                            pending_requests,
                            public_subscriptions,
                            private_subscriptions,
                        ).await;
                    }
                    Some(Ok(Message::Binary(bin))) => {
                        if let Ok(text) = String::from_utf8(bin.to_vec()) {
                            handle_incoming(
                                text,
                                pending_requests,
                                public_subscriptions,
                                private_subscriptions,
                            ).await;
                        } else {
                            warn!("Non-UTF8 binary message on {url}");
                        }
                    }
                    Some(Ok(Message::Ping(data))) => {
                        ws.send(Message::Pong(data)).await?;
                    }
                    Some(Ok(Message::Pong(_))) => {
                        // Pong received, connection is alive
                    }
                    Some(Ok(Message::Close(frame))) => {
                        warn!("WebSocket closed for {url}: {frame:?}");
                        return Ok(());
                    }
                    Some(Err(e)) => {
                        warn!("WebSocket error for {url}: {e}");
                        return Err(Box::new(e));
                    }
                    Some(Ok(Message::Frame(_))) => {
                        warn!("Received unsupported Frame message on {url}");
                    }
                    None => {
                        warn!("WebSocket stream ended for {url}");
                        return Ok(());
                    }
                }
            }

        _ = &mut read_deadline => {
            warn!("WebSocket read timeout for {url} - connection appears dead");
            return Err("websocket read timeout".into());
        }
        }
    }
}

async fn handle_incoming(
    text: String,
    pending_requests: &Arc<Mutex<HashMap<u64, ResponseSender>>>,
    public_subscriptions: &Arc<Mutex<HashMap<String, mpsc::UnboundedSender<String>>>>,
    private_subscriptions: &Arc<Mutex<HashMap<String, mpsc::UnboundedSender<String>>>>,
) {
    let parsed: Value = match serde_json::from_str(&text) {
        Ok(v) => v,
        Err(e) => {
            warn!("Failed to parse incoming message as JSON: {e}; raw: {text}");
            return;
        }
    };

    // RPC response: has "id" (including null)
    if let Some(id_value) = parsed.get("id") {
        // Handle messages with id: null (like subscription responses/errors)
        if id_value.is_null() {
            // Check if it's a subscription result or error
            if let Some(result) = parsed.get("result") {
                info!("Subscription confirmed: {result}");
            } else if let Some(error) = parsed.get("error") {
                warn!("Subscription error: {error}");
            }
            return;
        }

        // Handle messages with numeric IDs
        if let Some(id) = id_value.as_u64() {
            let mut pending = pending_requests.lock().await;
            if let Some(tx) = pending.remove(&id) {
                let _ = tx.send(text);
            }
            return;
        }
    }

    // Subscription notification: has "channel_name"
    if let Some(channel_name) = parsed.get("channel_name").and_then(|v| v.as_str()) {
        for route in [&private_subscriptions, &public_subscriptions] {
            let mut subs = route.lock().await;
            if let Some(sender) = subs.get_mut(channel_name) {
                if sender.send(text).is_err() {
                    // Receiver dropped; cleanup this subscription entry.
                    subs.remove(channel_name);
                }
                return;
            }
        }
        warn!("No subscription handler for channel: {channel_name}");
    }

    warn!("Received unhandled message: {text}");
}
