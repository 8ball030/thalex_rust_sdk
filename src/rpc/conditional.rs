use crate::{
    models::{
        CancelAllConditionalOrdersResponse, CancelConditionalOrderParams,
        CancelConditionalOrderResponse, ConditionalOrder, ConditionalOrdersResponse,
        CreateConditionalOrderParams, CreateConditionalOrderResponse,
    },
    types::ClientError,
    ws_client::WsClient,
};
use serde_json::Value;

pub struct ConditionalRpc<'a> {
    pub client: &'a WsClient,
}
impl<'a> ConditionalRpc<'a> {
    /// Conditional orders
    /// returns: Vec<ConditionalOrder>
    pub async fn conditional_orders(&self) -> Result<Vec<ConditionalOrder>, ClientError> {
        let result: Result<ConditionalOrdersResponse, ClientError> = self
            .client
            .send_rpc(
                "private/conditional_orders",
                serde_json::to_value(()).expect("Failed to serialize params"),
            )
            .await;
        match result {
            Ok(res) => match res {
                ConditionalOrdersResponse::ConditionalOrdersResult(res) => Ok(res.result),
                ConditionalOrdersResponse::RpcErrorResponse(err) => Err(ClientError::Rpc(err)),
            },
            Err(err) => Err(err),
        }
    }

    /// Create conditional order
    /// returns: ConditionalOrder
    pub async fn create_conditional_order(
        &self,
        params: CreateConditionalOrderParams,
    ) -> Result<ConditionalOrder, ClientError> {
        let result: Result<CreateConditionalOrderResponse, ClientError> = self
            .client
            .send_rpc(
                "private/create_conditional_order",
                serde_json::to_value(params).expect("Failed to serialize params"),
            )
            .await;
        match result {
            Ok(res) => match res {
                CreateConditionalOrderResponse::CreateConditionalOrderResult(res) => Ok(res.result),
                CreateConditionalOrderResponse::RpcErrorResponse(err) => Err(ClientError::Rpc(err)),
            },
            Err(err) => Err(err),
        }
    }

    /// Cancel conditional order
    /// returns: Value
    pub async fn cancel_conditional_order(
        &self,
        params: CancelConditionalOrderParams,
    ) -> Result<Value, ClientError> {
        let result: Result<CancelConditionalOrderResponse, ClientError> = self
            .client
            .send_rpc(
                "private/cancel_conditional_order",
                serde_json::to_value(params).expect("Failed to serialize params"),
            )
            .await;
        match result {
            Ok(res) => match res {
                CancelConditionalOrderResponse::CancelConditionalOrderResult(res) => Ok(res.result),
                CancelConditionalOrderResponse::RpcErrorResponse(err) => Err(ClientError::Rpc(err)),
            },
            Err(err) => Err(err),
        }
    }

    /// Bulk cancel conditional orders
    /// returns: Value
    pub async fn cancel_all_conditional_orders(&self) -> Result<Value, ClientError> {
        let result: Result<CancelAllConditionalOrdersResponse, ClientError> = self
            .client
            .send_rpc(
                "private/cancel_all_conditional_orders",
                serde_json::to_value(()).expect("Failed to serialize params"),
            )
            .await;
        match result {
            Ok(res) => match res {
                CancelAllConditionalOrdersResponse::CancelAllConditionalOrdersResult(res) => {
                    Ok(res.result)
                }
                CancelAllConditionalOrdersResponse::RpcErrorResponse(err) => {
                    Err(ClientError::Rpc(err))
                }
            },
            Err(err) => Err(err),
        }
    }
}
