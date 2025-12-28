use serde::{Deserialize, Serialize};

use crate::{manual_models::Resolution, models::RpcErrorResponse};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IndexDataPoint(
    pub f64, // time
    pub f64, // open
    pub f64, // high
    pub f64, // low
    pub f64, // close
);

pub type IndexPriceHistoricalData = Vec<IndexDataPoint>;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IndexPriceHistoricalDataRpcResult {
    pub index: Option<IndexPriceHistoricalData>,
    pub no_data: Option<bool>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IndexPriceHistoricalDataResult {
    #[serde(rename = "result")]
    pub result: IndexPriceHistoricalDataRpcResult,
    /// The request ID
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<u64>,
}
impl IndexPriceHistoricalDataResult {
    pub fn new(result: IndexPriceHistoricalDataRpcResult) -> IndexPriceHistoricalDataResult {
        IndexPriceHistoricalDataResult { result, id: None }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IndexPriceHistoricalDataResponse {
    IndexPriceHistoricalDataResult(IndexPriceHistoricalDataResult),
    RpcErrorResponse(RpcErrorResponse),
}

impl Default for IndexPriceHistoricalDataResponse {
    fn default() -> Self {
        Self::IndexPriceHistoricalDataResult(Default::default())
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IndexPriceHistoricalDataParams {
    /// Feedcode of the instrument (e.g. BTC-PERPETUAL).
    #[serde(rename = "index_name")]
    pub index_name: String,
    /// Start time (Unix timestamp).
    #[serde(rename = "from")]
    pub from: f64,
    /// End time (Unix timestamp) (exclusive).
    #[serde(rename = "to")]
    pub to: f64,
    /// Each data point will be aggregated using OHLC according to the specified resolution.
    #[serde(rename = "resolution")]
    pub resolution: Resolution,
}

impl IndexPriceHistoricalDataParams {
    pub fn new(
        index_name: String,
        from: f64,
        to: f64,
        resolution: Resolution,
    ) -> IndexPriceHistoricalDataParams {
        IndexPriceHistoricalDataParams {
            index_name,
            from,
            to,
            resolution,
        }
    }
}
