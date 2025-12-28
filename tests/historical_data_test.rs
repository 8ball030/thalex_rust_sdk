mod common;
use thalex_rust_sdk::{
    models::{MarkPriceHistoricalDataParams, mark_price_historical_data_params::Resolution},
    ws_client::WsClient,
};

const KNOWN_MARKET: &str = "BTC-PERPETUAL";
// dec 20, 2025 - dec 21, 2025
const FROM_UNIX_TS: f64 = 1760966400.0;
const TO_UNIX_TS: f64 = 1760969400.0;
params_rpc_test!(
    test_mark_price_historical_data,
    MarkPriceHistoricalDataParams {
        instrument_name: KNOWN_MARKET.to_string(),
        from: FROM_UNIX_TS,
        to: TO_UNIX_TS,
        resolution: Resolution::Variant15m,
    },
    mark_price_historical_data,
    "Market data instrument failure",
    historical_data,
    is_ok
);
