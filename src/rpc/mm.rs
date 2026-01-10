use crate::{
    models::{
        CancelMassQuoteParams, CancelMassQuoteResponse, DoubleSidedQuoteResult, MassQuoteParams,
        MassQuoteResponse, RpcErrorResponse, SetMmProtectionParams, SetMmProtectionResponse,
    },
    ws_client::WsClient,
};
use serde_json::Value;

pub struct MmRpc<'a> {
    pub client: &'a WsClient,
}
impl<'a> MmRpc<'a> {
    /// Send a mass quote
    /// returns: DoubleSidedQuoteResult
    pub async fn mass_quote(
        &self,
        params: MassQuoteParams,
    ) -> Result<DoubleSidedQuoteResult, RpcErrorResponse> {
        let result: MassQuoteResponse = self
            .client
            .send_rpc(
                "private/mass_quote",
                serde_json::to_value(params).expect("Failed to serialize params"),
            )
            .await
            .expect("Failed to send RPC request");
        match result {
            MassQuoteResponse::MassQuoteResult(res) => Ok(res.result),
            MassQuoteResponse::RpcErrorResponse(err) => Err(err),
        }
    }

    /// Bulk cancel mass quotes across all sessions
    /// returns: Value
    pub async fn cancel_mass_quote(
        &self,
        params: CancelMassQuoteParams,
    ) -> Result<Value, RpcErrorResponse> {
        let result: CancelMassQuoteResponse = self
            .client
            .send_rpc(
                "private/cancel_mass_quote",
                serde_json::to_value(params).expect("Failed to serialize params"),
            )
            .await
            .expect("Failed to send RPC request");
        match result {
            CancelMassQuoteResponse::CancelMassQuoteResult(res) => Ok(res.result),
            CancelMassQuoteResponse::RpcErrorResponse(err) => Err(err),
        }
    }

    /// Market maker protection configuration
    /// returns: Value
    pub async fn set_mm_protection(
        &self,
        params: SetMmProtectionParams,
    ) -> Result<Value, RpcErrorResponse> {
        let result: SetMmProtectionResponse = self
            .client
            .send_rpc(
                "private/set_mm_protection",
                serde_json::to_value(params).expect("Failed to serialize params"),
            )
            .await
            .expect("Failed to send RPC request");
        match result {
            SetMmProtectionResponse::SetMmProtectionResult(res) => Ok(res.result),
            SetMmProtectionResponse::RpcErrorResponse(err) => Err(err),
        }
    }
}
