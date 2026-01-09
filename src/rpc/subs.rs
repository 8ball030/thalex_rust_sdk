

use crate::{models::{
    SubscribeResponse,
    RpcErrorResponse,
    SubscribeParams,
    UnsubscribeRpcResult,
    OrderStatus,
    Ticker,
    Index,
    Instrument,
    UnsubscribeResponse,
    SubscribeRpcResult,
    UnsubscribeParams,
    UnsubscribeResult,
    SubscribeResult
}, ws_client::{
    WsClient,
}, types::{
    Error, 
}};
use serde_json::Value;

pub struct SubsRpc<'a> {
    pub client: &'a WsClient,
}
impl <'a> SubsRpc<'a> {

    /// Subscribe to private channels
    /// returns: SubscribeRpcResult
    pub async fn subscribe(&self, params: SubscribeParams) -> Result<SubscribeRpcResult, RpcErrorResponse> {
        let result: SubscribeResponse = self
            .client
            .send_rpc(
                "private/subscribe",
                serde_json::to_value(params).expect("Failed to serialize params"),
            )
            .await.expect("Failed to send RPC request");
        match result {
            SubscribeResponse::SubscribeResult(res) => Ok(res.result),
            SubscribeResponse::RpcErrorResponse(err) => Err(err),
        }
    }


    /// Subscribe to public channels
    /// returns: SubscribeRpcResult
    pub async fn subscribe(&self, params: SubscribeParams) -> Result<SubscribeRpcResult, RpcErrorResponse> {
        let result: SubscribeResponse = self
            .client
            .send_rpc(
                "public/subscribe",
                serde_json::to_value(params).expect("Failed to serialize params"),
            )
            .await.expect("Failed to send RPC request");
        match result {
            SubscribeResponse::SubscribeResult(res) => Ok(res.result),
            SubscribeResponse::RpcErrorResponse(err) => Err(err),
        }
    }


    /// Unsubscribe
    /// returns: UnsubscribeRpcResult
    pub async fn (&self, params: UnsubscribeParams) -> Result<UnsubscribeRpcResult, RpcErrorResponse> {
        let result: UnsubscribeResponse = self
            .client
            .send_rpc(
                "unsubscribe",
                serde_json::to_value(params).expect("Failed to serialize params"),
            )
            .await.expect("Failed to send RPC request");
        match result {
            UnsubscribeResponse::UnsubscribeResult(res) => Ok(res.result),
            UnsubscribeResponse::RpcErrorResponse(err) => Err(err),
        }
    }

    
}
