use crate::{
    manual_models::{
        historic_data_index::{
            IndexPriceHistoricalDataParams, IndexPriceHistoricalDataResponse,
            IndexPriceHistoricalDataRpcResult,
        },
        historic_data_mark::{
            MarkPriceHistoricalDataParams, MarkPriceHistoricalDataResponse,
            MarkPriceHistoricalDataRpcResult,
        },
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
    pub async fn index_price_historical_data(
        &self,
        params: IndexPriceHistoricalDataParams,
    ) -> Result<IndexPriceHistoricalDataRpcResult, RpcErrorResponse> {
        let result: IndexPriceHistoricalDataResponse = self
            .client
            .send_rpc(
                "public/index_price_historical_data",
                serde_json::to_value(params).expect("Failed to serialize params"),
            )
            .await
            .expect("Failed to send RPC request");
        match result {
            IndexPriceHistoricalDataResponse::IndexPriceHistoricalDataResult(res) => Ok(res.result),
            IndexPriceHistoricalDataResponse::RpcErrorResponse(err) => Err(err),
        }
    }
}
