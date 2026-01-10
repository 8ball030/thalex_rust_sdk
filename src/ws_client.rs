use dashmap::DashMap;
use serde::de::{DeserializeOwned, Deserializer};

use tokio::{
    sync::oneshot,
    time::{Duration, Instant, MissedTickBehavior, interval, sleep},
};

use futures_util::{SinkExt, StreamExt};
use log::{error, info, warn};
use serde_json::Value;
use std::env::var;
use std::sync::{
    Arc,
    atomic::{AtomicU64, Ordering},
};
use tokio::sync::{Mutex, mpsc, watch};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

use crate::{
    auth_utils::make_auth_token,
    models::{Instrument, RpcErrorResponse, RpcResponse},
    routing::RoutingVisitor,
    types::{
        ClientError, Error, ExternalEvent, InternalCommand, LoginState, RequestScope,
        ResponseSender, SubscribeResponse, SubscriptionChannel, WsStream,
    },
    utils::round_to_ticks,
};

use crate::channels::subscriptions::Subscriptions;
use crate::rpc::Rpc;

const URL: &str = "wss://thalex.com/ws/api/v2";
const PING_INTERVAL: Duration = Duration::from_secs(5);
const READ_TIMEOUT: Duration = Duration::from_secs(7);

pub struct WsClient {
    write_tx: mpsc::UnboundedSender<InternalCommand>,
    pending_requests: Arc<DashMap<u64, ResponseSender>>,
    pub public_subscriptions: Arc<DashMap<String, SubscriptionChannel>>,
    pub private_subscriptions: Arc<DashMap<String, SubscriptionChannel>>,
    next_id: Arc<AtomicU64>,
    shutdown_tx: watch::Sender<bool>,
    instruments_cache: Arc<DashMap<String, Instrument>>,
    login_state: LoginState,
    connection_state_rx: watch::Receiver<ExternalEvent>,
    current_connection_state: Arc<Mutex<ExternalEvent>>,
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

        let pending_requests = Arc::new(DashMap::new());
        let public_subscriptions = Arc::new(DashMap::new());
        let private_subscriptions = Arc::new(DashMap::new());
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
            instruments_cache: Arc::new(DashMap::new()),
            login_state,
            connection_state_rx,
            current_connection_state: Arc::new(Mutex::new(ExternalEvent::Disconnected)),
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
        self.instruments_cache.clear();
        for instrument in &instruments {
            self.instruments_cache.insert(
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
        let instrument = self.instruments_cache.get(instrument_name);
        // refresh cache if not found
        if let Some(instr) = instrument {
            Ok(round_to_ticks(price, instr.tick_size.unwrap()))
        } else {
            self.cache_instruments().await?;
            if let Some(instr) = self.instruments_cache.get(instrument_name) {
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
        T: serde::de::DeserializeOwned + std::fmt::Debug,
    {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);

        let (tx, rx) = oneshot::channel::<Arc<str>>();
        self.pending_requests.insert(id, tx);

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
            self.pending_requests.remove(&id);
            return Err(ClientError::Transport(Box::new(e)));
        }

        let response = rx.await?;

        // let as_success = serde_json::from_str::<crate::models::OrderHistoryResult>(&response);
        // let as_error = serde_json::from_str::<RpcErrorResponse>(&response);
        // println!("Success parse: {as_success:?}");
        // println!("Error parse:   {as_error:?}");
        let envelope: T = serde_json::from_str(&response)?;
        // .inspect_err(|e| println!("Failed to parse JSON: {}", e))?;
        // println!("RPC Response: {:?}", envelope);
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
                let (tx, mut rx) = mpsc::unbounded_channel::<Arc<str>>();

                {
                    match scope {
                        RequestScope::Public => {
                            self.public_subscriptions.insert(channel.clone(), tx);
                            info!("Subscribed to public channel: {channel}");
                        }
                        RequestScope::Private => {
                            self.private_subscriptions.insert(channel.clone(), tx);
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
            if self.public_subscriptions.remove(&channel).is_some() {
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
            if self.private_subscriptions.remove(&channel).is_some() {
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

    pub async fn resubscribe_all(&self) -> Result<(), Error> {
        let public_channels: Vec<String> = {
            self.public_subscriptions
                .iter()
                .map(|e| e.key().clone())
                .collect()
        };
        let _: RpcResponse = self
            .send_rpc(
                "public/subscribe",
                serde_json::json!({
                    "channels": public_channels
                }),
            )
            .await?;
        info!("Re-subscribed to public channels: {public_channels:?}");
        let private_channels: Vec<String> = {
            self.private_subscriptions
                .iter()
                .map(|e| e.key().clone())
                .collect()
        };
        let _: RpcResponse = self
            .send_rpc(
                "private/subscribe",
                serde_json::json!({
                    "channels": private_channels
                }),
            )
            .await?;
        info!("Re-subscribed to private channels: {private_channels:?}");
        Ok(())
    }
    pub async fn run_till_event(&self) -> ExternalEvent {
        let mut rx = self.connection_state_rx.clone();
        // ONLY return when state changes
        loop {
            if rx.changed().await.is_ok() {
                let state = *rx.borrow_and_update();
                if state != *self.current_connection_state.lock().await {
                    let mut current_state = self.current_connection_state.lock().await;
                    *current_state = state;
                    return state;
                }
            }
        }
    }

    pub fn is_connected(&self) -> bool {
        // Remove async - this is just reading a value
        *self.connection_state_rx.borrow() == ExternalEvent::Connected
    }

    pub async fn wait_for_connection(&self) {
        let mut rx = self.connection_state_rx.clone();

        // If already connected, return immediately
        if *rx.borrow_and_update() == ExternalEvent::Connected {
            let mut current_state = self.current_connection_state.lock().await;
            *current_state = ExternalEvent::Connected;
            return;
        }

        // Otherwise wait for state changes until connected
        while rx.changed().await.is_ok() {
            if *rx.borrow_and_update() == ExternalEvent::Connected {
                let mut current_state = self.current_connection_state.lock().await;
                *current_state = ExternalEvent::Connected;
                return;
            }
        }
    }
}

async fn connection_supervisor(
    url: String,
    mut cmd_rx: mpsc::UnboundedReceiver<InternalCommand>,
    mut shutdown_rx: watch::Receiver<bool>,
    pending_requests: Arc<DashMap<u64, ResponseSender>>,
    public_subscriptions: Arc<DashMap<String, SubscriptionChannel>>,
    private_subscriptions: Arc<DashMap<String, SubscriptionChannel>>,
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
                connection_state_tx.send(ExternalEvent::Connected).ok();
                info!("Connected to {url}");
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
                info!("Connection to {url} ended with result: {result:?}");

                if result.is_ok() {
                    connection_state_tx.send(ExternalEvent::Exited).ok();
                    info!("Connection exited normally for {url}");
                    break;
                }
                if let Err(e) = result {
                    connection_state_tx.send(ExternalEvent::Disconnected).ok();

                    error!("Connection error on {url}: {e}");
                }

                for key in pending_requests
                    .iter()
                    .map(|e| *e.key())
                    .collect::<Vec<u64>>()
                {
                    if let Some((_, tx)) = pending_requests.remove(&key) {
                        let _ = tx.send(r#"{"error":"connection closed"}"#.into());
                    }
                }

                if *shutdown_rx.borrow() {
                    connection_state_tx.send(ExternalEvent::Exited).ok();
                    info!("Shutdown after connection end for {url}");
                    break;
                }

                if cmd_rx.is_closed() {
                    connection_state_tx.send(ExternalEvent::Exited).ok();
                    info!("Command channel closed for {url}, stopping supervisor");
                    break;
                }

                info!("Reconnecting to {url} after backoff");
                tokio::time::sleep(std::time::Duration::from_secs(3)).await;
            }
            Err(e) => {
                error!("Failed to connect to {url}: {e}");
                if *shutdown_rx.borrow() || cmd_rx.is_closed() {
                    break;
                }
                tokio::time::sleep(std::time::Duration::from_secs(3)).await;
                connection_state_tx.send(ExternalEvent::Disconnected).ok();
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
    pending_requests: &Arc<DashMap<u64, ResponseSender>>,
    public_subscriptions: &Arc<DashMap<String, SubscriptionChannel>>,
    private_subscriptions: &Arc<DashMap<String, SubscriptionChannel>>,
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
                            text.as_ref(),
                            pending_requests,
                            public_subscriptions,
                            private_subscriptions,
                        ).await;
                    }
                    Some(Ok(Message::Binary(bin))) => {
                        if let Ok(text) = str::from_utf8(&bin) {
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

pub async fn handle_incoming(
    text: &str,
    pending_requests: &Arc<DashMap<u64, ResponseSender>>,
    public_subscriptions: &Arc<DashMap<String, SubscriptionChannel>>,
    private_subscriptions: &Arc<DashMap<String, SubscriptionChannel>>,
) {
    let mut deserializer = serde_json::Deserializer::from_str(text);

    let res = deserializer.deserialize_map(RoutingVisitor {
        text,
        pending_requests,
        public_subscriptions,
        private_subscriptions,
    });

    if let Err(e) = res {
        warn!("Failed to parse incoming message as JSON: {e}; raw: {text}");
    }
}
