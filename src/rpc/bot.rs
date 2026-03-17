use crate::{
    models::{
        Bot, BotsResponse, CancelAllBotsResponse, CancelBotParams, CancelBotResponse,
        CreateBotParams, CreateBotResponse,
    },
    types::ClientError,
    ws_client::WsClient,
};
use serde_json::Value;

pub struct BotRpc<'a> {
    pub client: &'a WsClient,
}
impl<'a> BotRpc<'a> {
    /// Get bots
    /// returns: Vec<Bot>
    pub async fn bots(&self) -> Result<Vec<Bot>, ClientError> {
        let result: Result<BotsResponse, ClientError> = self
            .client
            .send_rpc(
                "private/bots",
                serde_json::to_value(()).expect("Failed to serialize params"),
            )
            .await;
        match result {
            Ok(res) => match res {
                BotsResponse::BotsResult(res) => Ok(res.result),
                BotsResponse::RpcErrorResponse(err) => Err(ClientError::Rpc(err)),
            },
            Err(err) => Err(err),
        }
    }

    /// Create a bot
    /// returns: Bot
    pub async fn create_bot(&self, params: CreateBotParams) -> Result<Bot, ClientError> {
        let result: Result<CreateBotResponse, ClientError> = self
            .client
            .send_rpc(
                "private/create_bot",
                serde_json::to_value(params).expect("Failed to serialize params"),
            )
            .await;
        match result {
            Ok(res) => match res {
                CreateBotResponse::CreateBotResult(res) => Ok(res.result),
                CreateBotResponse::RpcErrorResponse(err) => Err(ClientError::Rpc(err)),
            },
            Err(err) => Err(err),
        }
    }

    /// Cancel a bot
    /// returns: Value
    pub async fn cancel_bot(&self, params: CancelBotParams) -> Result<Value, ClientError> {
        let result: Result<CancelBotResponse, ClientError> = self
            .client
            .send_rpc(
                "private/cancel_bot",
                serde_json::to_value(params).expect("Failed to serialize params"),
            )
            .await;
        match result {
            Ok(res) => match res {
                CancelBotResponse::CancelBotResult(res) => Ok(res.result),
                CancelBotResponse::RpcErrorResponse(err) => Err(ClientError::Rpc(err)),
            },
            Err(err) => Err(err),
        }
    }

    /// Cancel all bots
    /// returns: Value
    pub async fn cancel_all_bots(&self) -> Result<Value, ClientError> {
        let result: Result<CancelAllBotsResponse, ClientError> = self
            .client
            .send_rpc(
                "private/cancel_all_bots",
                serde_json::to_value(()).expect("Failed to serialize params"),
            )
            .await;
        match result {
            Ok(res) => match res {
                CancelAllBotsResponse::CancelAllBotsResult(res) => Ok(res.result),
                CancelAllBotsResponse::RpcErrorResponse(err) => Err(ClientError::Rpc(err)),
            },
            Err(err) => Err(err),
        }
    }
}
