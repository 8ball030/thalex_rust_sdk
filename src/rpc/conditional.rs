use crate::{
    models::{
        CancelAllConditionalOrdersResponse, CancelConditionalOrderParams,
        CancelConditionalOrderResponse, ConditionalOrder, ConditionalOrdersResponse,
        CreateConditionalOrderParams, CreateConditionalOrderResponse, RpcErrorResponse,
    },
    ws_client::WsClient,
};
use serde_json::Value;

pub struct ConditionalRpc<'a> {
    pub client: &'a WsClient,
}
impl<'a> ConditionalRpc<'a> {
    /// Conditional orders
    /// returns: Vec<ConditionalOrder>
    pub async fn conditional_orders(&self) -> Result<Vec<ConditionalOrder>, RpcErrorResponse> {
        let result: ConditionalOrdersResponse = self
            .client
            .send_rpc(
                "private/conditional_orders",
                serde_json::to_value(()).expect("Failed to serialize params"),
            )
            .await
            .expect("Failed to send RPC request");
        match result {
            ConditionalOrdersResponse::ConditionalOrdersResult(res) => Ok(res.result),
            ConditionalOrdersResponse::RpcErrorResponse(err) => Err(err),
        }
    }

    /// Create conditional order
    /// returns: ConditionalOrder
    pub async fn create_conditional_order(
        &self,
        params: CreateConditionalOrderParams,
    ) -> Result<ConditionalOrder, RpcErrorResponse> {
        let result: CreateConditionalOrderResponse = self
            .client
            .send_rpc(
                "private/create_conditional_order",
                serde_json::to_value(params).expect("Failed to serialize params"),
            )
            .await
            .expect("Failed to send RPC request");
        match result {
            CreateConditionalOrderResponse::CreateConditionalOrderResult(res) => Ok(res.result),
            CreateConditionalOrderResponse::RpcErrorResponse(err) => Err(err),
        }
    }

    /// Cancel conditional order
    /// returns: Value
    pub async fn cancel_conditional_order(
        &self,
        params: CancelConditionalOrderParams,
    ) -> Result<Value, RpcErrorResponse> {
        let result: CancelConditionalOrderResponse = self
            .client
            .send_rpc(
                "private/cancel_conditional_order",
                serde_json::to_value(params).expect("Failed to serialize params"),
            )
            .await
            .expect("Failed to send RPC request");
        match result {
            CancelConditionalOrderResponse::CancelConditionalOrderResult(res) => Ok(res.result),
            CancelConditionalOrderResponse::RpcErrorResponse(err) => Err(err),
        }
    }

    /// Bulk cancel conditional orders
    /// returns: Value
    pub async fn cancel_all_conditional_orders(&self) -> Result<Value, RpcErrorResponse> {
        let result: CancelAllConditionalOrdersResponse = self
            .client
            .send_rpc(
                "private/cancel_all_conditional_orders",
                serde_json::to_value(()).expect("Failed to serialize params"),
            )
            .await
            .expect("Failed to send RPC request");
        match result {
            CancelAllConditionalOrdersResponse::CancelAllConditionalOrdersResult(res) => {
                Ok(res.result)
            }
            CancelAllConditionalOrdersResponse::RpcErrorResponse(err) => Err(err),
        }
    }
}
