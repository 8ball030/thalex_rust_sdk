use crate::{
    models::{
        AmendParams, AmendResponse, BuyParams, BuyResponse, CancelAllResponse, CancelParams,
        CancelResponse, CancelSessionResponse, InsertParams, InsertResponse, OrderStatus,
        SellParams, SellResponse,
    },
    types::ClientError,
    ws_client::WsClient,
};
use rust_decimal::Decimal;
use serde_json::Value;

pub struct TradingRpc<'a> {
    pub client: &'a WsClient,
}
impl<'a> TradingRpc<'a> {
    /// Insert order
    /// returns: OrderStatus
    pub async fn insert(&self, params: InsertParams) -> Result<OrderStatus, ClientError> {
        let result: Result<InsertResponse, ClientError> = self
            .client
            .send_rpc(
                "private/insert",
                serde_json::to_value(params).expect("Failed to serialize params"),
            )
            .await;
        match result {
            Ok(res) => match res {
                InsertResponse::InsertResult(res) => Ok(res.result),
                InsertResponse::RpcErrorResponse(err) => Err(ClientError::Rpc(err)),
            },
            Err(err) => Err(err),
        }
    }

    /// Insert buy order
    /// returns: OrderStatus
    pub async fn buy(&self, params: BuyParams) -> Result<OrderStatus, ClientError> {
        let result: Result<BuyResponse, ClientError> = self
            .client
            .send_rpc(
                "private/buy",
                serde_json::to_value(params).expect("Failed to serialize params"),
            )
            .await;
        match result {
            Ok(res) => match res {
                BuyResponse::BuyResult(res) => Ok(res.result),
                BuyResponse::RpcErrorResponse(err) => Err(ClientError::Rpc(err)),
            },
            Err(err) => Err(err),
        }
    }

    /// Insert sell order
    /// returns: OrderStatus
    pub async fn sell(&self, params: SellParams) -> Result<OrderStatus, ClientError> {
        let result: Result<SellResponse, ClientError> = self
            .client
            .send_rpc(
                "private/sell",
                serde_json::to_value(params).expect("Failed to serialize params"),
            )
            .await;
        match result {
            Ok(res) => match res {
                SellResponse::SellResult(res) => Ok(res.result),
                SellResponse::RpcErrorResponse(err) => Err(ClientError::Rpc(err)),
            },
            Err(err) => Err(err),
        }
    }

    /// Amend order
    /// returns: OrderStatus
    pub async fn amend(&self, params: AmendParams) -> Result<OrderStatus, ClientError> {
        let result: Result<AmendResponse, ClientError> = self
            .client
            .send_rpc(
                "private/amend",
                serde_json::to_value(params).expect("Failed to serialize params"),
            )
            .await;
        match result {
            Ok(res) => match res {
                AmendResponse::AmendResult(res) => Ok(res.result),
                AmendResponse::RpcErrorResponse(err) => Err(ClientError::Rpc(err)),
            },
            Err(err) => Err(err),
        }
    }

    /// Cancel order
    /// returns: OrderStatus
    pub async fn cancel(&self, params: CancelParams) -> Result<OrderStatus, ClientError> {
        let result: Result<CancelResponse, ClientError> = self
            .client
            .send_rpc(
                "private/cancel",
                serde_json::to_value(params).expect("Failed to serialize params"),
            )
            .await;
        match result {
            Ok(res) => match res {
                CancelResponse::CancelResult(res) => Ok(res.result),
                CancelResponse::RpcErrorResponse(err) => Err(ClientError::Rpc(err)),
            },
            Err(err) => Err(err),
        }
    }

    /// Bulk cancel all orders
    /// returns: Decimal
    pub async fn cancel_all(&self) -> Result<Decimal, ClientError> {
        let result: Result<CancelAllResponse, ClientError> = self
            .client
            .send_rpc(
                "private/cancel_all",
                serde_json::to_value(()).expect("Failed to serialize params"),
            )
            .await;
        match result {
            Ok(res) => match res {
                CancelAllResponse::CancelAllResult(res) => Ok(res.result),
                CancelAllResponse::RpcErrorResponse(err) => Err(ClientError::Rpc(err)),
            },
            Err(err) => Err(err),
        }
    }

    /// Bulk cancel all orders in session
    /// returns: Value
    pub async fn cancel_session(&self) -> Result<Value, ClientError> {
        let result: Result<CancelSessionResponse, ClientError> = self
            .client
            .send_rpc(
                "private/cancel_session",
                serde_json::to_value(()).expect("Failed to serialize params"),
            )
            .await;
        match result {
            Ok(res) => match res {
                CancelSessionResponse::CancelSessionResult(res) => Ok(res.result),
                CancelSessionResponse::RpcErrorResponse(err) => Err(ClientError::Rpc(err)),
            },
            Err(err) => Err(err),
        }
    }
}
