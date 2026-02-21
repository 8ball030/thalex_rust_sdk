use serde::Deserialize;
use serde_json::Value;
use std::{fmt, str::FromStr};
use thiserror::Error;
use tokio::{net::TcpStream, sync::oneshot};
use tokio_tungstenite::{
    MaybeTlsStream, WebSocketStream,
    tungstenite::{Bytes, Message},
};

use crate::models::{ErrorResponse, RpcErrorResponse};

pub type WsStream = WebSocketStream<MaybeTlsStream<TcpStream>>;
pub type ResponseSender = oneshot::Sender<Bytes>;
pub type ChannelSender = tokio::sync::mpsc::UnboundedSender<Bytes>;
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = core::result::Result<T, Error>;

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
    pub private_key_pem: String,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Environment {
    Mainnet,
    Testnet,
}
impl Environment {
    pub fn get_url(&self) -> &str {
        match self {
            Environment::Mainnet => "wss://thalex.com/ws/api/v2",
            Environment::Testnet => "wss://testnet.thalex.com/ws/api/v2",
        }
    }
}
impl FromStr for Environment {
    type Err = ();
    fn from_str(env: &str) -> core::result::Result<Self, Self::Err> {
        match env.to_lowercase().as_str() {
            "mainnet" => Ok(Environment::Mainnet),
            "testnet" => Ok(Environment::Testnet),
            _ => Err(()),
        }
    }
}
