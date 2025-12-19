use serde::Deserialize;
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
use crate::models::{ErrorResponse, Instrument, PrivateTradeHistoryResult, PublicInstruments};

use crate::ws::Subscriptions;

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
    pub id: u64,
    pub result: PublicInstruments,
    pub error: Option<ErrorResponse>,
}

pub struct WsClient {
    write_tx: mpsc::UnboundedSender<InternalCommand>,
    pending_requests: Arc<Mutex<HashMap<u64, ResponseSender>>>,
    // channel_name -> mpsc::UnboundedSender<String>
    pub subscriptions: Arc<Mutex<HashMap<String, mpsc::UnboundedSender<String>>>>,
    next_id: Arc<AtomicU64>,
    shutdown_tx: watch::Sender<bool>,
}

impl WsClient {
    /// Create a client and start the supervisor loop, connecting to the default URL.
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
        };

        // Spawn supervisor that reconnects and owns the websocket.
        tokio::spawn(connection_supervisor(
            url,
            cmd_rx,
            shutdown_rx,
            pending_requests,
            subscriptions,
        ));

        Ok(client)
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

        let msg = serde_json::json!({
            "method": "public/login",
            "params": {
                "token": token,
                "account": account_id
            }
        });

        self.send_json(msg)?;

        info!("Sent login message");
        Ok(())
    }

    /// JSON-RPC style call: sends a method/params, waits for matching `id`.
    pub async fn get_instruments(&self) -> Result<Vec<Instrument>, Error> {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);

        let (tx, rx) = oneshot::channel::<String>();
        {
            let mut pending = self.pending_requests.lock().await;
            pending.insert(id, tx);
        }

        let request = serde_json::json!({
            "jsonrpc": "2.0",
            "id": id,
            "method": "public/instruments",
            "params": {}
        });

        let text = request.to_string();

        if let Err(e) = self
            .write_tx
            .send(InternalCommand::Send(Message::Text(text.into())))
        {
            let mut pending = self.pending_requests.lock().await;
            pending.remove(&id);
            return Err(Box::new(e));
        }

        let response = rx.await?;

        let parsed: RpcMessage = serde_json::from_str(&response)?;

        let result: PublicInstruments = parsed.result;

        let instruments = match result {
            PublicInstruments::PublicInstrumentsResult(v) => v,
            PublicInstruments::ErrorResponse(err) => {
                return Err(Box::new(std::io::Error::other(format!(
                    "API error: {err:?}"
                ))));
            }
        };

        Ok(instruments)
    }

    pub async fn get_trade_history(
        &self,
        bookmark: Option<String>,
    ) -> Result<PrivateTradeHistoryResult, Error> {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);

        let (tx, rx) = oneshot::channel::<String>();
        {
            let mut pending = self.pending_requests.lock().await;
            pending.insert(id, tx);
        }

        let mut request = serde_json::json!({
            "jsonrpc": "2.0",
            "id": id,
            "method": "private/trade_history",
            "params": {
            }
        });
        if let Some(bm) = bookmark {
            request["params"]["bookmark"] = serde_json::Value::String(bm);
        }

        let text = request.to_string();

        if let Err(e) = self
            .write_tx
            .send(InternalCommand::Send(Message::Text(text.into())))
        {
            let mut pending = self.pending_requests.lock().await;
            pending.remove(&id);
            return Err(Box::new(e));
        }

        let response = rx.await?;

        let parsed_response: Value = serde_json::from_str(&response)?;

        let parsed: PrivateTradeHistoryResult =
            serde_json::from_value(parsed_response["result"].clone())?;

        Ok(parsed)
    }

    /// The callback runs in its own task and receives each message for this channel.
    pub fn subscriptions(&self) -> Subscriptions {
        Subscriptions { client: self }
    }
    pub async fn unsubscribe(&self, channel: &str) -> Result<(), Error> {
        let channel = channel.to_string();

        {
            let mut subs = self.subscriptions.lock().await;
            subs.remove(&channel);
        }

        let msg = serde_json::json!({
            "method": "unsubscribe",
            "params": {
                "channels": [channel]
            }
        });

        self.send_json(msg)?;

        info!("Unsubscribed from channel: {channel}");
        Ok(())
    }

    /// Request clean shutdown of the websocket supervisor.
    pub async fn shutdown(&self, reason: &'static str) -> Result<(), Error> {
        info!("Shutdown requested: {reason}");
        let _ = self.shutdown_tx.send(true);
        let _ = self.write_tx.send(InternalCommand::Close);
        Ok(())
    }

    pub fn send_json(&self, value: Value) -> Result<(), Error> {
        let text = value.to_string();
        self.write_tx
            .send(InternalCommand::Send(Message::Text(text.into())))?;
        Ok(())
    }
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
