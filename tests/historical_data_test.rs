mod common;
use thalex_rust_sdk::{
    manual_models::{
        Resolution, historic_data_index::IndexPriceHistoricalDataParams,
        historic_data_mark::MarkPriceHistoricalDataParams,
    },
    ws_client::WsClient,
};

const KNOWN_INDEX: &str = "BTCUSD";
const KNOWN_MARKET: &str = "BTC-PERPETUAL";
const KNOWN_FUTURE: &str = "BTC-21OCT25";
const KNOWN_OPTION: &str = "BTC-21OCT25-105000-C";
const FROM_UNIX_TS: f64 = 1760966400.0;
const TO_UNIX_TS: f64 = 1760969400.0;

macro_rules! mark_price_test {
    ($name:ident, $instrument:expr, $resolution:expr, $error_msg:expr) => {
        params_rpc_test!(
            $name,
            MarkPriceHistoricalDataParams {
                instrument_name: $instrument.to_string(),
                from: FROM_UNIX_TS,
                to: TO_UNIX_TS,
                resolution: $resolution,
            },
            mark_price_historical_data,
            $error_msg,
            historical_data,
            is_ok
        );
    };
}
macro_rules! index_price_test {
    ($name:ident, $instrument:expr, $resolution:expr, $error_msg:expr) => {
        params_rpc_test!(
            $name,
            IndexPriceHistoricalDataParams {
                index_name: $instrument.to_string(),
                from: FROM_UNIX_TS,
                to: TO_UNIX_TS,
                resolution: $resolution,
            },
            index_price_historical_data,
            $error_msg,
            historical_data,
            is_ok
        );
    };
}

mark_price_test!(
    test_mark_price_historical_data_15m,
    KNOWN_MARKET,
    Resolution::Variant15m,
    "Market data instrument failure"
);
mark_price_test!(
    test_mark_price_historical_data_1m,
    KNOWN_MARKET,
    Resolution::Variant1m,
    "Market data instrument failure"
);
mark_price_test!(
    test_mark_price_historical_data_future_1m,
    KNOWN_FUTURE,
    Resolution::Variant1m,
    "Market data future failure"
);
mark_price_test!(
    test_mark_price_historical_data_future_15m,
    KNOWN_FUTURE,
    Resolution::Variant15m,
    "Market data future failure"
);
mark_price_test!(
    test_mark_price_historical_data_option_1m,
    KNOWN_OPTION,
    Resolution::Variant1m,
    "Market data option failure"
);
mark_price_test!(
    test_mark_price_historical_data_option_15m,
    KNOWN_OPTION,
    Resolution::Variant15m,
    "Market data option failure"
);

index_price_test!(
    test_index_price_historical_data_15m,
    KNOWN_INDEX,
    Resolution::Variant15m,
    "Index data instrument failure"
);
index_price_test!(
    test_index_price_historical_data_1m,
    KNOWN_INDEX,
    Resolution::Variant1m,
    "Index data instrument failure"
);
