use serde::Deserialize;
use serde::de::DeserializeOwned;
use std::fmt;
use tokio::{net::TcpStream, sync::oneshot};

use futures_util::{SinkExt, StreamExt};
use log::{error, info, warn};
use serde_json::Value;
use std::collections::HashMap;
use std::env::var;
use std::sync::{
    Arc,
    atomic::{AtomicU64, Ordering},
};
use tokio::sync::{Mutex, mpsc, watch};
use tokio_tungstenite::{
    MaybeTlsStream, WebSocketStream, connect_async, tungstenite::protocol::Message,
};

use crate::auth_utils::make_auth_token;
use crate::models::order_status::{Direction, OrderType};
use crate::models::{
    ErrorResponse, Instrument, OrderStatus, PortfolioEntry, PrivatePortfolio,
    PrivateTradeHistoryResult, PublicInstruments,
};

use crate::channels::subscriptions::Subscriptions;

type WsStream = WebSocketStream<MaybeTlsStream<TcpStream>>;
type ResponseSender = oneshot::Sender<String>;
type Error = Box<dyn std::error::Error + Send + Sync>;

const URL: &str = "wss://thalex.com/ws/api/v2";

/// Commands sent from the client API to the connection task.
enum InternalCommand {
    Send(Message),
    Close,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RpcMessage {
    pub id: Option<u64>,
    pub result: Value,
    pub error: Option<ErrorResponse>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RequestScope {
    Public,
    Private,
}

impl fmt::Display for RequestScope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RequestScope::Public => write!(f, "public"),
            RequestScope::Private => write!(f, "private"),
        }
    }
}

pub struct WsClient {
    write_tx: mpsc::UnboundedSender<InternalCommand>,
    pending_requests: Arc<Mutex<HashMap<u64, ResponseSender>>>,
    // channel_name -> mpsc::UnboundedSender<String>
    pub subscriptions: Arc<Mutex<HashMap<String, mpsc::UnboundedSender<String>>>>,
    next_id: Arc<AtomicU64>,
    shutdown_tx: watch::Sender<bool>,
    instruments_cache: Arc<Mutex<HashMap<String, Instrument>>>,
}

impl WsClient {
    pub fn subscriptions(&self) -> Subscriptions {
        Subscriptions { client: self }
    }
    pub async fn connect_default() -> Result<Self, Error> {
        Self::connect(URL).await
    }

    pub async fn from_env() -> Result<Self, Error> {
        let key_path = var("THALEX_PRIVATE_KEY_PATH").unwrap();
        let key_id = var("THALEX_KEY_ID").unwrap();
        let account_id = var("THALEX_ACCOUNT_ID").unwrap();
        let client = WsClient::connect_default().await?;
        client.login(&key_id, &account_id, &key_path).await?;
        Ok(client)
    }

    /// Create a client and start the supervisor loop, connecting to the given URL.
    pub async fn connect(url: impl Into<String>) -> Result<Self, Error> {
        let url = url.into();

        let (cmd_tx, cmd_rx) = mpsc::unbounded_channel::<InternalCommand>();
        let (shutdown_tx, shutdown_rx) = watch::channel(false);

        let pending_requests = Arc::new(Mutex::new(HashMap::new()));
        let subscriptions = Arc::new(Mutex::new(HashMap::new()));
        let next_id = Arc::new(AtomicU64::new(1));

        let client = WsClient {
            // url: url.clone(),
            write_tx: cmd_tx.clone(),
            pending_requests: pending_requests.clone(),
            subscriptions: subscriptions.clone(),
            next_id: next_id.clone(),
            shutdown_tx: shutdown_tx.clone(),
            instruments_cache: Arc::new(Mutex::new(HashMap::new())),
        };

        // Spawn supervisor that reconnects and owns the websocket.
        tokio::spawn(connection_supervisor(
            url,
            cmd_rx,
            shutdown_rx,
            pending_requests,
            subscriptions,
        ));
        client.cache_instruments().await?;
        Ok(client)
    }

    async fn cache_instruments(&self) -> Result<(), Error> {
        let instruments = self.get_instruments().await?;
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

    async fn check_and_refresh_instrument_cache(
        &self,
        instrument_name: &str,
    ) -> Result<Instrument, Error> {
        let instrument = self
            .instruments_cache
            .lock()
            .await
            .get(instrument_name)
            .cloned();
        // refresh cache if not found?
        if let Some(instr) = instrument {
            Ok(instr)
        } else {
            self.cache_instruments().await?;
            let cache = self.instruments_cache.lock().await;
            if let Some(instr) = cache.get(instrument_name).cloned() {
                Ok(instr)
            } else {
                Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    format!("Instrument not found: {instrument_name}"),
                )))
            }
        }
    }

    pub fn send_json(&self, value: Value) -> Result<(), Error> {
        let text = value.to_string();
        self.write_tx
            .send(InternalCommand::Send(Message::Text(text.into())))?;
        Ok(())
    }
    pub async fn send_rpc<T: DeserializeOwned>(
        &self,
        method: &str,
        params: serde_json::Value,
    ) -> Result<T, Error> {
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

        // info!("Sending RPC request: {}", text);

        if let Err(e) = self
            .write_tx
            .send(InternalCommand::Send(Message::Text(text.into())))
        {
            let mut pending = self.pending_requests.lock().await;
            pending.remove(&id);
            return Err(Box::new(e));
        }

        let response = rx.await?;

        // info!("Received RPC response: {}", response);

        let parsed: RpcMessage = serde_json::from_str(&response)?;

        // This assumes `RpcMessage` has a `result` field compatible with T
        // info!("RPC response for id={id}: {:?}", parsed);
        let result: T = serde_json::from_value(parsed.result)?;

        Ok(result)
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
    ) -> Result<Vec<String>, Error>
    where
        P: DeserializeOwned + Send + 'static,
        F: FnMut(P) + Send + 'static,
    {
        let (tx, mut rx) = mpsc::unbounded_channel::<String>();

        {
            let mut subs = self.subscriptions.lock().await;
            subs.insert(channel.clone(), tx);
        }
        let sub_result: Vec<String> = serde_json::from_value(
            self.send_rpc::<Value>(
                &format!("{scope}/subscribe"),
                serde_json::json!({
                    "channels": [channel.clone()]
                }),
            )
            .await?,
        )?;
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
        info!("Subscription result: {sub_result:?}");
        Ok(sub_result)
    }

    pub async fn unsubscribe(&self, channel: &str) -> Result<(), Error> {
        let channel = channel.to_string();

        {
            let mut subs = self.subscriptions.lock().await;
            subs.remove(&channel);
        }

        let _ = self
            .send_rpc::<Value>(
                "public/unsubscribe",
                serde_json::json!({
                    "channels": [channel.clone()]
                }),
            )
            .await?;
        info!("Unsubscribed from channel: {channel}");
        Ok(())
    }
    pub async fn login(
        &self,
        key_id: &str,
        account_id: &str,
        private_key_path: &str,
    ) -> Result<(), Error> {
        // we read the private key from the given file path
        let private_key_pem = tokio::fs::read_to_string(private_key_path).await?;
        let token = make_auth_token(key_id, private_key_pem)?;
        let result: Value = self
            .send_rpc(
                "public/login",
                serde_json::json!({
                    "token": token,
                    "account": account_id
                }),
            )
            .await?;

        info!("Sent login message, received response: {result:?}");
        Ok(())
    }

    /// Get instruments using the generic RPC method
    pub async fn get_instruments(&self) -> Result<Vec<Instrument>, Error> {
        let result: PublicInstruments = self
            .send_rpc("public/instruments", serde_json::json!({}))
            .await?;

        match result {
            PublicInstruments::PublicInstrumentsResult(v) => Ok(v),
            PublicInstruments::ErrorResponse(err) => Err(Box::new(std::io::Error::other(format!(
                "API error: {err:?}"
            )))),
        }
    }

    pub async fn get_trade_history(
        &self,
        bookmark: Option<String>,
    ) -> Result<PrivateTradeHistoryResult, Error> {
        let result: Value = self
            .send_rpc(
                "private/trade_history",
                if let Some(bm) = bookmark {
                    serde_json::json!({ "bookmark": bm })
                } else {
                    serde_json::json!({})
                },
            )
            .await?;
        let parsed: PrivateTradeHistoryResult = serde_json::from_value(result)?;

        Ok(parsed)
    }

    pub async fn get_positions(&self) -> Result<Vec<PortfolioEntry>, Error> {
        let result: Value = self
            .send_rpc("private/portfolio", serde_json::json!({}))
            .await?;
        // We match the result to see if it's an error or a valid portfolio
        let parsed: PrivatePortfolio = serde_json::from_value(result)?;
        let positions = match parsed {
            PrivatePortfolio::PrivatePortfolioResult(v) => v,
            PrivatePortfolio::ErrorResponse(err) => {
                return Err(Box::new(std::io::Error::other(format!(
                    "API error: {err:?}"
                ))));
            }
        };
        Ok(positions)
    }

    pub async fn set_cancel_on_disconnect(&self) -> Result<(), Error> {
        let result = self
            .send_rpc::<Value>(
                "private/set_cancel_on_disconnect",
                serde_json::json!({ "timeout_secs": 6}),
            )
            .await;
        info!("Set cancel_on_disconnect result: {result:?}");
        Ok(())
    }
    #[allow(clippy::too_many_arguments)]
    pub async fn insert_order(
        &self,
        instrument_name: &str,
        amount: f64,
        price: f64,
        direction: Direction,
        order_type: OrderType,
        post_only: bool,
        reject_post_only: bool,
    ) -> Result<OrderStatus, Error> {
        // Check if instrument is already cached
        let instrument = self
            .check_and_refresh_instrument_cache(instrument_name)
            .await?;
        let tick_size = instrument.tick_size.unwrap();

        let result: Value = self
            .send_rpc(
                "private/insert",
                serde_json::json!({
                    "instrument_name": instrument_name,
                    "amount": amount,
                    "price": round_to_ticks(price, tick_size),
                    "direction": direction,
                    "order_type": order_type,
                    "post_only": post_only,
                    "reject_post_only": reject_post_only

                }),
            )
            .await?;

        let order_status: OrderStatus = serde_json::from_value(result)?;
        Ok(order_status)
    }
    pub async fn amend_order(
        &self,
        order_id: String,
        instrument_name: &str,
        amount: f64,
        price: f64,
    ) -> Result<OrderStatus, Error> {
        let instrument = self
            .check_and_refresh_instrument_cache(instrument_name)
            .await?;
        let tick_size = instrument.tick_size.unwrap();
        let result: Value = self
            .send_rpc(
                "private/amend",
                serde_json::json!({
                    "order_id": order_id,
                    "amount": amount,
                    "price": round_to_ticks(price, tick_size)
                }),
            )
            .await?;

        let order_status: OrderStatus = serde_json::from_value(result)?;
        Ok(order_status)
    }
}

fn round_to_ticks(price: f64, tick_size: f64) -> f64 {
    (price / tick_size).round() * tick_size
}

/// Supervisor: reconnects on failures, replays subscriptions on each new connection.
async fn connection_supervisor(
    url: String,
    mut cmd_rx: mpsc::UnboundedReceiver<InternalCommand>,
    mut shutdown_rx: watch::Receiver<bool>,
    pending_requests: Arc<Mutex<HashMap<u64, ResponseSender>>>,
    subscriptions: Arc<Mutex<HashMap<String, mpsc::UnboundedSender<String>>>>,
) {
    info!("Connection supervisor started for {url}");

    loop {
        if *shutdown_rx.borrow() {
            info!("Supervisor sees shutdown for {url}");
            break;
        }

        match connect_async(&url).await {
            Ok((ws_stream, _)) => {
                info!("WebSocket connected to {url}");

                let result = run_single_connection(
                    &url,
                    ws_stream,
                    &mut cmd_rx,
                    &mut shutdown_rx,
                    &pending_requests,
                    &subscriptions,
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
                if *shutdown_rx.borrow() || cmd_rx.is_closed() {
                    break;
                }
                tokio::time::sleep(std::time::Duration::from_secs(3)).await;
            }
        }
    }

    info!("Connection supervisor exited for {url}");
}

/// Single connection lifetime. Exits on close / error / shutdown.
async fn run_single_connection(
    url: &str,
    mut ws: WsStream,
    cmd_rx: &mut mpsc::UnboundedReceiver<InternalCommand>,
    shutdown_rx: &mut watch::Receiver<bool>,
    pending_requests: &Arc<Mutex<HashMap<u64, ResponseSender>>>,
    subscriptions: &Arc<Mutex<HashMap<String, mpsc::UnboundedSender<String>>>>,
) -> Result<(), Error> {
    // Re-subscribe active channels on new connection.
    {
        let subs = subscriptions.lock().await;
        for channel in subs.keys() {
            let msg = serde_json::json!({
                "method": "public/subscribe",
                "params": { "channels": [channel] },
            });
            ws.send(Message::Text(msg.to_string().into())).await?;
        }
    }

    loop {
        tokio::select! {
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
                match msg {
                    Some(Ok(Message::Text(text))) => {
                        handle_incoming(text.to_string(), pending_requests, subscriptions).await;
                    }
                    Some(Ok(Message::Binary(bin))) => {
                        if let Ok(text) = String::from_utf8(bin.to_vec()) {
                            handle_incoming(text, pending_requests, subscriptions).await;
                        } else {
                            warn!("Non-UTF8 binary message on {url}");
                        }
                    }
                    Some(Ok(Message::Ping(data))) => {
                        ws.send(Message::Pong(data)).await?;
                    }
                    Some(Ok(Message::Pong(_))) => {
                        // ignore
                    }
                    Some(Ok(Message::Close(frame))) => {
                        warn!("WebSocket closed for {url}: {frame:?}");
                        return Ok(());
                    }
                    Some(Err(e)) => {
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
        }
    }
}

async fn handle_incoming(
    text: String,
    pending_requests: &Arc<Mutex<HashMap<u64, ResponseSender>>>,
    subscriptions: &Arc<Mutex<HashMap<String, mpsc::UnboundedSender<String>>>>,
) {
    let parsed: Value = match serde_json::from_str(&text) {
        Ok(v) => v,
        Err(e) => {
            warn!("Failed to parse incoming message as JSON: {e}; raw: {text}");
            return;
        }
    };

    // RPC response: has "id"
    if let Some(id) = parsed.get("id").and_then(|v| v.as_u64()) {
        let mut pending = pending_requests.lock().await;
        if let Some(tx) = pending.remove(&id) {
            let _ = tx.send(text);
        } else {
            warn!("Received RPC response for unknown id={id}");
        }
        return;
    }

    // Subscription notification: has "channel_name"
    if let Some(channel_name) = parsed.get("channel_name").and_then(|v| v.as_str()) {
        let mut subs = subscriptions.lock().await;
        if let Some(sender) = subs.get_mut(channel_name) {
            if sender.send(text).is_err() {
                // Receiver dropped; cleanup this subscription entry.
                subs.remove(channel_name);
            }
        } else {
            warn!("Received message for unsubscribed channel: {channel_name}");
        }
        return;
    }

    warn!("Received unhandled message: {text}");
}
