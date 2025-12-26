use crate::{
    models::{
        AllInstrumentsParams, AllInstrumentsResponse, BookParams, BookResponse, BookRpcResult,
        Index, IndexParams, IndexResponse, Instrument, InstrumentParams, InstrumentResponse,
        InstrumentsResponse, RpcErrorResponse, Ticker, TickerParams, TickerResponse,
    },
    ws_client::WsClient,
};

pub struct MarketDataRpc<'a> {
    pub client: &'a WsClient,
}
impl<'a> MarketDataRpc<'a> {
    /// Active instruments
    /// returns: Vec<Instrument>
    pub async fn instruments(&self) -> Result<Vec<Instrument>, RpcErrorResponse> {
        let result: InstrumentsResponse = self
            .client
            .send_rpc(
                "public/instruments",
                serde_json::to_value(()).expect("Failed to serialize params"),
            )
            .await
            .expect("Failed to send RPC request");
        match result {
            InstrumentsResponse::InstrumentsResult(res) => Ok(res.result),
            InstrumentsResponse::RpcErrorResponse(err) => Err(err),
        }
    }

    /// All instruments
    /// returns: Vec<Instrument>
    pub async fn all_instruments(
        &self,
        params: AllInstrumentsParams,
    ) -> Result<Vec<Instrument>, RpcErrorResponse> {
        let result: AllInstrumentsResponse = self
            .client
            .send_rpc(
                "public/all_instruments",
                serde_json::to_value(params).expect("Failed to serialize params"),
            )
            .await
            .expect("Failed to send RPC request");
        match result {
            AllInstrumentsResponse::AllInstrumentsResult(res) => Ok(res.result),
            AllInstrumentsResponse::RpcErrorResponse(err) => Err(err),
        }
    }

    /// Single instrument
    /// returns: Instrument
    pub async fn instrument(
        &self,
        params: InstrumentParams,
    ) -> Result<Instrument, RpcErrorResponse> {
        let result: InstrumentResponse = self
            .client
            .send_rpc(
                "public/instrument",
                serde_json::to_value(params).expect("Failed to serialize params"),
            )
            .await
            .expect("Failed to send RPC request");
        match result {
            InstrumentResponse::InstrumentResult(res) => Ok(res.result),
            InstrumentResponse::RpcErrorResponse(err) => Err(err),
        }
    }

    /// Single ticker value
    /// returns: Ticker
    pub async fn ticker(&self, params: TickerParams) -> Result<Ticker, RpcErrorResponse> {
        let result: TickerResponse = self
            .client
            .send_rpc(
                "public/ticker",
                serde_json::to_value(params).expect("Failed to serialize params"),
            )
            .await
            .expect("Failed to send RPC request");
        match result {
            TickerResponse::TickerResult(res) => Ok(res.result),
            TickerResponse::RpcErrorResponse(err) => Err(err),
        }
    }

    /// Single index value
    /// returns: Index
    pub async fn index(&self, params: IndexParams) -> Result<Index, RpcErrorResponse> {
        let result: IndexResponse = self
            .client
            .send_rpc(
                "public/index",
                serde_json::to_value(params).expect("Failed to serialize params"),
            )
            .await
            .expect("Failed to send RPC request");
        match result {
            IndexResponse::IndexResult(res) => Ok(res.result),
            IndexResponse::RpcErrorResponse(err) => Err(err),
        }
    }

    /// Single order book
    /// returns: BookRpcResult
    pub async fn book(&self, params: BookParams) -> Result<BookRpcResult, RpcErrorResponse> {
        let result: BookResponse = self
            .client
            .send_rpc(
                "public/book",
                serde_json::to_value(params).expect("Failed to serialize params"),
            )
            .await
            .expect("Failed to send RPC request");
        match result {
            BookResponse::BookResult(res) => Ok(res.result),
            BookResponse::RpcErrorResponse(err) => Err(err),
        }
    }
}
