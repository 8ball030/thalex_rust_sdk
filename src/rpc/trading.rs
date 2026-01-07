use crate::{
    models::{
        AmendParams, AmendResponse, BuyParams, BuyResponse, CancelAllResponse, CancelParams,
        CancelResponse, CancelSessionResponse, InsertParams, InsertResponse, OrderStatus,
        RpcErrorResponse, SellParams, SellResponse,
    },
    ws_client::WsClient,
};
use serde_json::Value;

pub struct TradingRpc<'a> {
    pub client: &'a WsClient,
}
impl<'a> TradingRpc<'a> {
    /// Insert order
    /// returns: OrderStatus
    pub async fn insert(&self, params: InsertParams) -> Result<OrderStatus, RpcErrorResponse> {
        let result: InsertResponse = self
            .client
            .send_rpc(
                "private/insert",
                serde_json::to_value(params).expect("Failed to serialize params"),
            )
            .await
            .expect("Failed to send RPC request");
        match result {
            InsertResponse::InsertResult(res) => Ok(res.result),
            InsertResponse::RpcErrorResponse(err) => Err(err),
        }
    }

    /// Insert buy order
    /// returns: OrderStatus
    pub async fn buy(&self, params: BuyParams) -> Result<OrderStatus, RpcErrorResponse> {
        let result: BuyResponse = self
            .client
            .send_rpc(
                "private/buy",
                serde_json::to_value(params).expect("Failed to serialize params"),
            )
            .await
            .expect("Failed to send RPC request");
        match result {
            BuyResponse::BuyResult(res) => Ok(res.result),
            BuyResponse::RpcErrorResponse(err) => Err(err),
        }
    }

    /// Insert sell order
    /// returns: OrderStatus
    pub async fn sell(&self, params: SellParams) -> Result<OrderStatus, RpcErrorResponse> {
        let result: SellResponse = self
            .client
            .send_rpc(
                "private/sell",
                serde_json::to_value(params).expect("Failed to serialize params"),
            )
            .await
            .expect("Failed to send RPC request");
        match result {
            SellResponse::SellResult(res) => Ok(res.result),
            SellResponse::RpcErrorResponse(err) => Err(err),
        }
    }

    /// Amend order
    /// returns: OrderStatus
    pub async fn amend(&self, params: AmendParams) -> Result<OrderStatus, RpcErrorResponse> {
        let result: AmendResponse = self
            .client
            .send_rpc(
                "private/amend",
                serde_json::to_value(params).expect("Failed to serialize params"),
            )
            .await
            .expect("Failed to send RPC request");
        match result {
            AmendResponse::AmendResult(res) => Ok(res.result),
            AmendResponse::RpcErrorResponse(err) => Err(err),
        }
    }

    /// Cancel order
    /// returns: OrderStatus
    pub async fn cancel(&self, params: CancelParams) -> Result<OrderStatus, RpcErrorResponse> {
        let result: CancelResponse = self
            .client
            .send_rpc(
                "private/cancel",
                serde_json::to_value(params).expect("Failed to serialize params"),
            )
            .await
            .expect("Failed to send RPC request");
        match result {
            CancelResponse::CancelResult(res) => Ok(res.result),
            CancelResponse::RpcErrorResponse(err) => Err(err),
        }
    }

    /// Bulk cancel all orders
    /// returns: f64
    pub async fn cancel_all(&self) -> Result<f64, RpcErrorResponse> {
        let result: CancelAllResponse = self
            .client
            .send_rpc(
                "private/cancel_all",
                serde_json::to_value(()).expect("Failed to serialize params"),
            )
            .await
            .expect("Failed to send RPC request");
        match result {
            CancelAllResponse::CancelAllResult(res) => Ok(res.result),
            CancelAllResponse::RpcErrorResponse(err) => Err(err),
        }
    }

    /// Bulk cancel all orders in session
    /// returns: Value
    pub async fn cancel_session(&self) -> Result<Value, RpcErrorResponse> {
        let result: CancelSessionResponse = self
            .client
            .send_rpc(
                "private/cancel_session",
                serde_json::to_value(()).expect("Failed to serialize params"),
            )
            .await
            .expect("Failed to send RPC request");
        match result {
            CancelSessionResponse::CancelSessionResult(res) => Ok(res.result),
            CancelSessionResponse::RpcErrorResponse(err) => Err(err),
        }
    }
}
