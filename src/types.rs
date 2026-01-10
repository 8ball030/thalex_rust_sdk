use serde::Deserialize;
use serde_json::Value;
use std::fmt;
use thiserror::Error;
use tokio::{net::TcpStream, sync::oneshot};
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, tungstenite::Message};

use crate::models::{ErrorResponse, RpcErrorResponse};

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

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum SubscribeResponse {
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
