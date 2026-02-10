use dashmap::DashMap;
use serde::de::DeserializeOwned;

use tokio::{
    sync::oneshot,
    task::JoinHandle,
    time::{Duration, Instant, MissedTickBehavior, interval, sleep},
};

use futures_util::{SinkExt, StreamExt};
use log::{debug, error, info, warn};
use serde_json::Value;
use std::env::var;
use std::sync::{
    Arc,
    atomic::{AtomicU64, Ordering},
};
use tokio::sync::{Mutex, mpsc, watch};
use tokio_tungstenite::{
    connect_async,
    tungstenite::{Bytes, protocol::Message},
};

use crate::{
    auth_utils::make_auth_token,
    models::{Instrument, RpcErrorResponse, RpcResponse},
    routing::{extract_channel, extract_id},
    types::{
        ChannelSender, ClientError, Environment, Error, ExternalEvent, InternalCommand, LoginState,
        RequestScope, ResponseSender, SubscribeResponse, WsStream,
    },
    utils::round_to_ticks,
};

use crate::channels::subscriptions::Subscriptions;
use crate::rpc::Rpc;
use std::str::FromStr;

const PING_INTERVAL: Duration = Duration::from_secs(5);
const READ_TIMEOUT: Duration = Duration::from_secs(7);

pub struct WsClient {
    pub write_tx: mpsc::UnboundedSender<InternalCommand>,
    pending_requests: Arc<DashMap<u64, ResponseSender>>,
    pub public_subscriptions: Arc<DashMap<String, ChannelSender>>,
    pub private_subscriptions: Arc<DashMap<String, ChannelSender>>,
    next_id: Arc<AtomicU64>,
    shutdown_tx: watch::Sender<bool>,
    pub instruments_cache: Arc<DashMap<String, Instrument>>,
    login_state: LoginState,
    connection_state_rx: watch::Receiver<ExternalEvent>,
    current_connection_state: Arc<Mutex<ExternalEvent>>,
    supervisor_handle: Arc<Mutex<Option<JoinHandle<()>>>>,
    subscription_tasks: Arc<Mutex<Vec<JoinHandle<()>>>>,
}

#[inline(always)]
pub fn deserialise_to_type<T>(s: &Bytes) -> Result<T, serde_json::Error>
where
    T: DeserializeOwned,
{
    serde_json::from_slice::<T>(s)
}

impl WsClient {
    pub fn subscriptions(&self) -> Subscriptions<'_> {
        Subscriptions { client: self }
    }

    pub fn rpc(&self) -> Rpc<'_> {
        Rpc { client: self }
    }

    pub async fn from_env() -> Result<Self, Error> {
        let key_path = var("THALEX_PRIVATE_KEY_PATH").unwrap();
        let key_id = var("THALEX_KEY_ID").unwrap();
        let account_id = var("THALEX_ACCOUNT_ID").unwrap();
        let env_str = var("THALEX_ENVIRONMENT")
            .expect("THALEX_ENVIRONMENT not set. Must be 'Mainnet' or 'Testnet'");
        let env = Environment::from_str(&env_str).expect("Invalid THALEX_ENVIRONMENT value");
        let client = WsClient::new(env, key_id, account_id, key_path).await?;
        client.wait_for_connection().await;
        info!("WsClient created from environment variables Logging in...");
        client.login().await.expect("Login failed");
        Ok(client)
    }

    pub async fn new_public(env: Environment) -> Result<Self, Error> {
        let client = WsClient::new(env, "".to_string(), "".to_string(), "".to_string()).await?;
        client.wait_for_connection().await;
        Ok(client)
    }

    pub async fn new(
        env: Environment,
        key_id: String,
        account_id: String,
        key_path: String,
    ) -> Result<Self, Error> {
        let url = env.get_url();

        let (cmd_tx, cmd_rx) = mpsc::unbounded_channel::<InternalCommand>();
        let (shutdown_tx, shutdown_rx) = watch::channel(false);

        let pending_requests = Arc::new(DashMap::new());
        let public_subscriptions: Arc<DashMap<String, ChannelSender>> = Arc::new(DashMap::new());
        let private_subscriptions: Arc<DashMap<String, ChannelSender>> = Arc::new(DashMap::new());
        let next_id = Arc::new(AtomicU64::new(1));

        let (connection_state_tx, mut connection_state_rx) =
            watch::channel(ExternalEvent::Disconnected);
        connection_state_rx.mark_unchanged();

        let login_state = LoginState {
            key_id,
            account_id,
            key_path,
        };

        let supervisor_handle = tokio::spawn(connection_supervisor(
            url.to_string(),
            cmd_rx,
            shutdown_rx,
            pending_requests.clone(),
            public_subscriptions.clone(),
            private_subscriptions.clone(),
            connection_state_tx.clone(),
        ));

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
            supervisor_handle: Arc::new(Mutex::new(Some(supervisor_handle))),
            subscription_tasks: Arc::new(Mutex::new(Vec::new())),
        };

        client.cache_instruments().await?;
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

        let (tx, rx) = oneshot::channel::<Bytes>();
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

        let envelope: T = deserialise_to_type(&response)?;
        Ok(envelope)
    }

    pub async fn shutdown(&self, reason: &'static str) -> Result<(), Error> {
        debug!("Shutdown requested: {reason}");
        self.public_subscriptions.clear();
        self.private_subscriptions.clear();
        let _ = self.shutdown_tx.send(true);
        let _ = self.write_tx.send(InternalCommand::Close);
        if let Some(handle) = self.supervisor_handle.lock().await.take() {
            match tokio::time::timeout(Duration::from_secs(5), handle).await {
                Ok(Ok(())) => {
                    debug!("Supervisor task completed successfully");
                }
                Ok(Err(e)) => {
                    error!("Supervisor task panicked: {e:?}");
                    return Err("Supervisor task panicked".into());
                }
                Err(_) => {
                    error!("Supervisor task timeout after 5s");
                    return Err("Supervisor shutdown timeout".into());
                }
            }
        }
        for task in self.subscription_tasks.lock().await.drain(..) {
            task.abort();
        }
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
        let (tx, mut rx) = mpsc::unbounded_channel::<Bytes>();

        {
            match scope {
                RequestScope::Public => {
                    self.public_subscriptions.insert(channel.clone(), tx);
                    debug!("Subscribing to public channel: {channel}");
                }
                RequestScope::Private => {
                    self.private_subscriptions.insert(channel.clone(), tx);
                    debug!("Subscribing to private channel: {channel}");
                }
            }
        }

        let handle = tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                let parsed: P = match deserialise_to_type(&msg) {
                    Ok(m) => m,
                    Err(e) => {
                        warn!("Failed to parse channel message: {e}; raw: {msg:?}");
                        continue;
                    }
                };

                callback(parsed);
            }
        });
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
                debug!("Subscribed to channel: {channel}");
                self.subscription_tasks.lock().await.push(handle);
                Ok(channel)
            }
            SubscribeResponse::Err { error, id: _id } => {
                warn!("Subscription error: {error:?}");
                match scope {
                    RequestScope::Public => {
                        self.public_subscriptions.remove(&channel);
                    }
                    RequestScope::Private => {
                        self.private_subscriptions.remove(&channel);
                    }
                }
                handle.abort();
                Err(ClientError::Rpc(error))
            }
        }
    }

    pub async fn unsubscribe(&self, channel: &str) -> Result<(), Error> {
        let channel = channel.to_string();
        if let Some(task) = self.subscription_tasks.lock().await.pop() {
            task.abort();
        }
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
        debug!("Sent login message, received response: {result:?}");
        if let Some(error) = result.get("error") {
            warn!("Login error: {error:?}");
            Err(Box::new(std::io::Error::other(format!(
                "Login error: {error:?}"
            ))))
        } else {
            debug!("Login successful");
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
        debug!("Set cancel_on_disconnect result: {result:?}");
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
        debug!("Re-subscribed to public channels: {public_channels:?}");
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
        debug!("Re-subscribed to private channels: {private_channels:?}");
        Ok(())
    }

    pub async fn run_till_event(&self) -> ExternalEvent {
        let mut rx = self.connection_state_rx.clone();

        loop {
            if rx.changed().await.is_ok() {
                let state = *rx.borrow_and_update();
                let mut current_state = self.current_connection_state.lock().await;
                if state != *current_state {
                    info!("Connection state changed to: {:?}", state);
                    *current_state = state;
                    return state;
                }
            } else {
                return ExternalEvent::Exited;
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
    public_subscriptions: Arc<DashMap<String, ChannelSender>>,
    private_subscriptions: Arc<DashMap<String, ChannelSender>>,
    connection_state_tx: watch::Sender<ExternalEvent>,
) {
    debug!("Connection supervisor started for {url}");

    let mut attempts: u64 = 1;
    loop {
        if *shutdown_rx.borrow() {
            info!("Supervisor sees shutdown for {url}");
            break;
        }

        debug!("Attempting to connect to {url} (attempt {attempts})");
        match connect_async(&url).await {
            Ok((ws_stream, _)) => {
                connection_state_tx.send(ExternalEvent::Connected).ok();
                attempts = 1;
                debug!("Connected to {url}");
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
                debug!("Connection to {url} ended with result: {result:?}");

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

                connection_state_tx.send(ExternalEvent::Disconnected).ok();
                let cooldown_secs = attempts * 3;
                debug!("Reconnecting to {url} in {cooldown_secs} seconds (attempt {attempts})");
                tokio::time::sleep(std::time::Duration::from_secs(cooldown_secs)).await;
            }
            Err(e) => {
                connection_state_tx.send(ExternalEvent::Disconnected).ok();
                error!("Failed to connect to {url}: {e} on attempt {attempts}");
                if *shutdown_rx.borrow() || cmd_rx.is_closed() {
                    break;
                }
                tokio::time::sleep(std::time::Duration::from_secs(attempts * 3)).await;
                attempts += 1;
            }
        }
    }

    debug!("Connection supervisor exited for {url}");
}

async fn run_single_connection(
    url: &str,
    mut ws: WsStream,
    cmd_rx: &mut mpsc::UnboundedReceiver<InternalCommand>,
    shutdown_rx: &mut watch::Receiver<bool>,
    pending_requests: &Arc<DashMap<u64, ResponseSender>>,
    public_subscriptions: &Arc<DashMap<String, ChannelSender>>,
    private_subscriptions: &Arc<DashMap<String, ChannelSender>>,
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
                        return Err("websocket closed by command".into());
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
                            text.into(),
                            pending_requests,
                            public_subscriptions,
                            private_subscriptions,
                        );
                    }
                    Some(Ok(Message::Binary(bin))) => {
                        handle_incoming(
                            bin,
                            pending_requests,
                            public_subscriptions,
                            private_subscriptions,
                        );
                    }
                    Some(Ok(Message::Ping(data))) => {
                        ws.send(Message::Pong(data)).await?;
                    }
                    Some(Ok(Message::Pong(_))) => {
                        // Pong received, connection is alive
                    }
                    Some(Ok(Message::Close(frame))) => {
                        warn!("WebSocket closed for {url}: {frame:?}");
                        return Err("websocket closed".into());
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

#[inline(always)]
pub fn handle_incoming(
    bytes: Bytes,
    pending_requests: &Arc<DashMap<u64, ResponseSender>>,
    public_subscriptions: &Arc<DashMap<String, ChannelSender>>,
    private_subscriptions: &Arc<DashMap<String, ChannelSender>>,
) {
    if check_and_send_pending_request(&bytes, pending_requests) {
        return;
    }
    if check_and_send_subscription_message(&bytes, private_subscriptions) {
        return;
    }
    if check_and_send_subscription_message(&bytes, public_subscriptions) {}
}

#[inline(always)]
fn check_and_send_pending_request(
    bytes: &Bytes,
    pending_requests: &Arc<DashMap<u64, ResponseSender>>,
) -> bool {
    if let Some(id) = extract_id(bytes)
        && let Some((_, tx)) = pending_requests.remove(&id)
    {
        let _ = tx.send(bytes.clone());
        return true;
    }
    false
}

#[inline(always)]
fn check_and_send_subscription_message(
    bytes: &Bytes,
    subscriptions: &Arc<DashMap<String, ChannelSender>>,
) -> bool {
    if let Some(channel) = extract_channel(bytes)
        && let Some(sender) = subscriptions.get(channel)
    {
        if sender.send(bytes.clone()).is_err() {
            subscriptions.remove(channel);
        }
        return true;
    }
    false
}
