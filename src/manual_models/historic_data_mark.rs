use serde::{Deserialize, Serialize};

use crate::{manual_models::Resolution, models::RpcErrorResponse};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TopOfBook(
    pub Option<f64>, // bid_price
    pub Option<f64>, // bid_size
    pub Option<f64>, // ask_price
    pub Option<f64>, // ask_size
);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PerpetualDataPoint(
    pub f64,                                                                 // time
    pub f64,                                                                 // open
    pub f64,                                                                 // high
    pub f64,                                                                 // low
    pub f64,                                                                 // close
    pub f64,                                                                 // funding_payment
    #[serde(skip_serializing_if = "Option::is_none")] pub Option<TopOfBook>, // top_of_book
);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FuturesAndCombosDataPoint(
    pub f64,                                                                 // time
    pub f64,                                                                 // open
    pub f64,                                                                 // high
    pub f64,                                                                 // low
    pub f64,                                                                 // close
    #[serde(skip_serializing_if = "Option::is_none")] pub Option<TopOfBook>, // top_of_book
);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OptionsDataPoint(
    pub f64,                                                                 // time
    pub f64,                                                                 // open
    pub f64,                                                                 // high
    pub f64,                                                                 // low
    pub f64,                                                                 // close
    pub f64,                                                                 // open_iv
    pub f64,                                                                 // high_iv
    pub f64,                                                                 // low_iv
    pub f64,                                                                 // close_iv
    #[serde(skip_serializing_if = "Option::is_none")] pub Option<TopOfBook>, // top_of_book
);

pub type PerpetualMarkPriceHistoricalData = Vec<PerpetualDataPoint>;
pub type FuturesAndCombinationsMarkPriceHistoricalData = Vec<FuturesAndCombosDataPoint>;
pub type OptionsMarkPriceHistoricalData = Vec<OptionsDataPoint>;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MarkPriceData {
    Perpetual(PerpetualMarkPriceHistoricalData),
    FuturesAndCombinations(FuturesAndCombinationsMarkPriceHistoricalData),
    Options(OptionsMarkPriceHistoricalData),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MarkPriceHistoricalDataRpcResult {
    pub instrument_type: InstrumentType,
    pub mark: Option<MarkPriceData>,
    pub no_data: Option<bool>,
}

#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Default, Ord, PartialOrd, Hash, Serialize, Deserialize,
)]
pub enum InstrumentType {
    #[default]
    #[serde(rename = "perpetual")]
    Perpetual,
    #[serde(rename = "future")]
    Future,
    #[serde(rename = "combination")]
    Combination,
    #[serde(rename = "option")]
    Option,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MarkPriceHistoricalDataResult {
    #[serde(rename = "result")]
    pub result: MarkPriceHistoricalDataRpcResult,
    /// The request ID
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<u64>,
}
impl MarkPriceHistoricalDataResult {
    pub fn new(result: MarkPriceHistoricalDataRpcResult) -> MarkPriceHistoricalDataResult {
        MarkPriceHistoricalDataResult { result, id: None }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MarkPriceHistoricalDataResponse {
    MarkPriceHistoricalDataResult(MarkPriceHistoricalDataResult),
    RpcErrorResponse(RpcErrorResponse),
}

impl Default for MarkPriceHistoricalDataResponse {
    fn default() -> Self {
        Self::MarkPriceHistoricalDataResult(Default::default())
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MarkPriceHistoricalDataParams {
    /// Feedcode of the instrument (e.g. BTC-PERPETUAL).
    #[serde(rename = "instrument_name")]
    pub instrument_name: String,
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

impl MarkPriceHistoricalDataParams {
    pub fn new(
        instrument_name: String,
        from: f64,
        to: f64,
        resolution: Resolution,
    ) -> MarkPriceHistoricalDataParams {
        MarkPriceHistoricalDataParams {
            instrument_name,
            from,
            to,
            resolution,
        }
    }
}
