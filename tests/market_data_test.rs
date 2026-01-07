mod common;
use thalex_rust_sdk::{
    models::{AllInstrumentsParams, BookParams, IndexParams, InstrumentParams, TickerParams},
    ws_client::WsClient,
};

const KNOWN_INSTRUMENT: &str = "BTC-PERPETUAL";
const KNOWN_UNDERLYING: &str = "BTCUSD";
const UNKNOWN_INSTRUMENT: &str = "NOT_EXISTING";

no_params_rpc_test!(
    test_instruments,
    instruments,
    "Market data instruments",
    market_data,
    is_ok
);

params_rpc_test!(
    test_all_instruments,
    AllInstrumentsParams::default(),
    all_instruments,
    "Market data all_instruments",
    market_data,
    is_ok
);

params_rpc_test!(
    test_instrument,
    InstrumentParams {
        instrument_name: KNOWN_INSTRUMENT.to_string(),
    },
    instrument,
    "Market data instrument",
    market_data,
    is_ok
);

params_rpc_test!(
    test_ticker,
    TickerParams {
        instrument_name: KNOWN_INSTRUMENT.to_string(),
    },
    ticker,
    "Market data ticker",
    market_data,
    is_ok
);

params_rpc_test!(
    test_index,
    IndexParams {
        underlying: KNOWN_UNDERLYING.to_string(),
    },
    index,
    "Market data index",
    market_data,
    is_ok
);

params_rpc_test!(
    test_book,
    BookParams {
        instrument_name: KNOWN_INSTRUMENT.to_string(),
    },
    book,
    "Market data book",
    market_data,
    is_ok
);

params_rpc_test!(
    test_instrument_failure,
    InstrumentParams {
        instrument_name: UNKNOWN_INSTRUMENT.to_string(),
    },
    instrument,
    "Market data instrument failure",
    market_data,
    is_err
);

params_rpc_test!(
    test_ticker_failure,
    TickerParams {
        instrument_name: UNKNOWN_INSTRUMENT.to_string(),
    },
    ticker,
    "Market data ticker failure",
    market_data,
    is_err
);

params_rpc_test!(
    test_book_failure,
    BookParams {
        instrument_name: UNKNOWN_INSTRUMENT.to_string(),
    },
    book,
    "Market data book failure",
    market_data,
    is_err
);
