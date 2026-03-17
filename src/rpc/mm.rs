use crate::{
    models::{
        CancelMassQuoteParams, CancelMassQuoteResponse, DoubleSidedQuoteResult, MassQuoteParams,
        MassQuoteResponse, SetMmProtectionParams, SetMmProtectionResponse,
    },
    types::ClientError,
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
    ) -> Result<DoubleSidedQuoteResult, ClientError> {
        let result: Result<MassQuoteResponse, ClientError> = self
            .client
            .send_rpc(
                "private/mass_quote",
                serde_json::to_value(params).expect("Failed to serialize params"),
            )
            .await;
        match result {
            Ok(res) => match res {
                MassQuoteResponse::MassQuoteResult(res) => Ok(res.result),
                MassQuoteResponse::RpcErrorResponse(err) => Err(ClientError::Rpc(err)),
            },
            Err(err) => Err(err),
        }
    }

    /// Bulk cancel mass quotes across all sessions
    /// returns: Value
    pub async fn cancel_mass_quote(
        &self,
        params: CancelMassQuoteParams,
    ) -> Result<Value, ClientError> {
        let result: Result<CancelMassQuoteResponse, ClientError> = self
            .client
            .send_rpc(
                "private/cancel_mass_quote",
                serde_json::to_value(params).expect("Failed to serialize params"),
            )
            .await;
        match result {
            Ok(res) => match res {
                CancelMassQuoteResponse::CancelMassQuoteResult(res) => Ok(res.result),
                CancelMassQuoteResponse::RpcErrorResponse(err) => Err(ClientError::Rpc(err)),
            },
            Err(err) => Err(err),
        }
    }

    /// Market maker protection configuration
    /// returns: Value
    pub async fn set_mm_protection(
        &self,
        params: SetMmProtectionParams,
    ) -> Result<Value, ClientError> {
        let result: Result<SetMmProtectionResponse, ClientError> = self
            .client
            .send_rpc(
                "private/set_mm_protection",
                serde_json::to_value(params).expect("Failed to serialize params"),
            )
            .await;
        match result {
            Ok(res) => match res {
                SetMmProtectionResponse::SetMmProtectionResult(res) => Ok(res.result),
                SetMmProtectionResponse::RpcErrorResponse(err) => Err(ClientError::Rpc(err)),
            },
            Err(err) => Err(err),
        }
    }
}
