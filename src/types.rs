use serde::Deserialize;
use serde_json::Value;
use std::fmt;
use tokio::{net::TcpStream, sync::oneshot};
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, tungstenite::Message};

use crate::models::ErrorResponse;

pub type WsStream = WebSocketStream<MaybeTlsStream<TcpStream>>;
pub type ResponseSender = oneshot::Sender<String>;
pub type Error = Box<dyn std::error::Error + Send + Sync>;

pub enum InternalCommand {
    Send(Message),
    Close,
}

#[derive(PartialEq, Clone, Debug, Copy)]
pub enum ExternalEvent {
    Connected,
    Disconnected,
    Exited,
}

#[derive(Clone, Debug)]
pub struct LoginState {
    pub key_id: String,
    pub account_id: String,
    pub key_path: String,
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
