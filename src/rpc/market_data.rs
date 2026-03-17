use crate::{
    models::{
        AllInstrumentsParams, AllInstrumentsResponse, BookParams, BookResponse, BookRpcResult,
        Index, IndexParams, IndexResponse, Instrument, InstrumentParams, InstrumentResponse,
        InstrumentsResponse, Ticker, TickerParams, TickerResponse,
    },
    types::ClientError,
    ws_client::WsClient,
};

pub struct MarketDataRpc<'a> {
    pub client: &'a WsClient,
}
impl<'a> MarketDataRpc<'a> {
    /// Active instruments
    /// returns: Vec<Instrument>
    pub async fn instruments(&self) -> Result<Vec<Instrument>, ClientError> {
        let result: Result<InstrumentsResponse, ClientError> = self
            .client
            .send_rpc(
                "public/instruments",
                serde_json::to_value(()).expect("Failed to serialize params"),
            )
            .await;
        match result {
            Ok(res) => match res {
                InstrumentsResponse::InstrumentsResult(res) => Ok(res.result),
                InstrumentsResponse::RpcErrorResponse(err) => Err(ClientError::Rpc(err)),
            },
            Err(err) => Err(err),
        }
    }

    /// All instruments
    /// returns: Vec<Instrument>
    pub async fn all_instruments(
        &self,
        params: AllInstrumentsParams,
    ) -> Result<Vec<Instrument>, ClientError> {
        let result: Result<AllInstrumentsResponse, ClientError> = self
            .client
            .send_rpc(
                "public/all_instruments",
                serde_json::to_value(params).expect("Failed to serialize params"),
            )
            .await;
        match result {
            Ok(res) => match res {
                AllInstrumentsResponse::AllInstrumentsResult(res) => Ok(res.result),
                AllInstrumentsResponse::RpcErrorResponse(err) => Err(ClientError::Rpc(err)),
            },
            Err(err) => Err(err),
        }
    }

    /// Single instrument
    /// returns: Instrument
    pub async fn instrument(&self, params: InstrumentParams) -> Result<Instrument, ClientError> {
        let result: Result<InstrumentResponse, ClientError> = self
            .client
            .send_rpc(
                "public/instrument",
                serde_json::to_value(params).expect("Failed to serialize params"),
            )
            .await;
        match result {
            Ok(res) => match res {
                InstrumentResponse::InstrumentResult(res) => Ok(res.result),
                InstrumentResponse::RpcErrorResponse(err) => Err(ClientError::Rpc(err)),
            },
            Err(err) => Err(err),
        }
    }

    /// Single ticker value
    /// returns: Ticker
    pub async fn ticker(&self, params: TickerParams) -> Result<Ticker, ClientError> {
        let result: Result<TickerResponse, ClientError> = self
            .client
            .send_rpc(
                "public/ticker",
                serde_json::to_value(params).expect("Failed to serialize params"),
            )
            .await;
        match result {
            Ok(res) => match res {
                TickerResponse::TickerResult(res) => Ok(res.result),
                TickerResponse::RpcErrorResponse(err) => Err(ClientError::Rpc(err)),
            },
            Err(err) => Err(err),
        }
    }

    /// Single index value
    /// returns: Index
    pub async fn index(&self, params: IndexParams) -> Result<Index, ClientError> {
        let result: Result<IndexResponse, ClientError> = self
            .client
            .send_rpc(
                "public/index",
                serde_json::to_value(params).expect("Failed to serialize params"),
            )
            .await;
        match result {
            Ok(res) => match res {
                IndexResponse::IndexResult(res) => Ok(res.result),
                IndexResponse::RpcErrorResponse(err) => Err(ClientError::Rpc(err)),
            },
            Err(err) => Err(err),
        }
    }

    /// Single order book
    /// returns: BookRpcResult
    pub async fn book(&self, params: BookParams) -> Result<BookRpcResult, ClientError> {
        let result: Result<BookResponse, ClientError> = self
            .client
            .send_rpc(
                "public/book",
                serde_json::to_value(params).expect("Failed to serialize params"),
            )
            .await;
        match result {
            Ok(res) => match res {
                BookResponse::BookResult(res) => Ok(res.result),
                BookResponse::RpcErrorResponse(err) => Err(ClientError::Rpc(err)),
            },
            Err(err) => Err(err),
        }
    }
}
