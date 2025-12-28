use crate::{
    historic_data_models::{
        MarkPriceHistoricalDataParams, MarkPriceHistoricalDataResponse,
        MarkPriceHistoricalDataRpcResult,
    },
    models::RpcErrorResponse,
    ws_client::WsClient,
};

pub struct HistoricalDataRpc<'a> {
    pub client: &'a WsClient,
}
impl<'a> HistoricalDataRpc<'a> {
    /// Mark price historical data.
    /// returns: MarkPriceHistoricalDataRpcResult
    pub async fn mark_price_historical_data(
        &self,
        params: MarkPriceHistoricalDataParams,
    ) -> Result<MarkPriceHistoricalDataRpcResult, RpcErrorResponse> {
        let result: MarkPriceHistoricalDataResponse = self
            .client
            .send_rpc(
                "public/mark_price_historical_data",
                serde_json::to_value(params).expect("Failed to serialize params"),
            )
            .await
            .expect("Failed to send RPC request");
        match result {
            MarkPriceHistoricalDataResponse::MarkPriceHistoricalDataResult(res) => Ok(res.result),
            MarkPriceHistoricalDataResponse::RpcErrorResponse(err) => Err(err),
        }
    }
}
