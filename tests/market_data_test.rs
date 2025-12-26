use thalex_rust_sdk::{
    models::{AllInstrumentsParams, BookParams, IndexParams, InstrumentParams, TickerParams},
    ws_client::WsClient,
};

const KNOWN_INSTRUMENT: &str = "BTC-PERPETUAL";
const KNOWN_UNDERLYING: &str = "BTCUSD";
const UNKNOWN_INSTRUMENT: &str = "NOT_EXISTING";

#[tokio::test]
async fn test_instruments() {
    let client = WsClient::new_public().await.unwrap();
    let result = client.rpc().market_data().instruments().await;
    assert!(
        result.is_ok(),
        "Market data instruments failed: {:?}",
        result.err()
    );
    client.shutdown("Test complete").await.unwrap();
}

/// Test all_instruments RPC method
#[tokio::test]
async fn test_all_instruments() {
    let client = WsClient::new_public().await.unwrap();
    let params = AllInstrumentsParams::default();
    let result = client.rpc().market_data().all_instruments(params).await;
    assert!(
        result.is_ok(),
        "Market data all_instruments failed: {:?}",
        result.err()
    );
    client.shutdown("Test complete").await.unwrap();
}

/// Test instrument RPC method
#[tokio::test]
async fn test_instrument() {
    let client = WsClient::new_public().await.unwrap();
    let params = InstrumentParams {
        instrument_name: KNOWN_INSTRUMENT.to_string(),
    };
    let result = client.rpc().market_data().instrument(params).await;
    assert!(
        result.is_ok(),
        "Market data instrument failed: {:?}",
        result.err()
    );
    client.shutdown("Test complete").await.unwrap();
}

/// Test ticker
#[tokio::test]
async fn test_ticker() {
    let client = WsClient::new_public().await.unwrap();
    let params = TickerParams {
        instrument_name: KNOWN_INSTRUMENT.to_string(),
    };
    let result = client.rpc().market_data().ticker(params).await;
    assert!(
        result.is_ok(),
        "Market data ticker failed: {:?}",
        result.err()
    );
    client.shutdown("Test complete").await.unwrap();
}

/// Test index
#[tokio::test]
async fn test_index() {
    let client = WsClient::new_public().await.unwrap();
    let params = IndexParams {
        underlying: KNOWN_UNDERLYING.to_string(),
    };
    let result = client.rpc().market_data().index(params).await;
    assert!(
        result.is_ok(),
        "Market data index failed: {:?}",
        result.err()
    );
    client.shutdown("Test complete").await.unwrap();
}

#[tokio::test]
async fn test_book() {
    let client = WsClient::new_public().await.unwrap();
    let params = BookParams {
        instrument_name: KNOWN_INSTRUMENT.to_string(),
    };
    let result = client.rpc().market_data().book(params).await;
    assert!(
        result.is_ok(),
        "Market data indices failed: {:?}",
        result.err()
    );
    client.shutdown("Test complete").await.unwrap();
}

/// Failure cases
#[tokio::test]
async fn test_instrument_failure() {
    let client = WsClient::new_public().await.unwrap();
    let params = InstrumentParams {
        instrument_name: UNKNOWN_INSTRUMENT.to_string(),
    };
    let result = client.rpc().market_data().instrument(params).await;
    assert!(result.is_err(), "Expected market data instrument to fail");
    client.shutdown("Test complete").await.unwrap();
}

#[tokio::test]
async fn test_ticker_failure() {
    let client = WsClient::new_public().await.unwrap();
    let params = TickerParams {
        instrument_name: UNKNOWN_INSTRUMENT.to_string(),
    };
    let result = client.rpc().market_data().ticker(params).await;
    assert!(result.is_err(), "Expected market data ticker to fail");
    client.shutdown("Test complete").await.unwrap();
}

#[tokio::test]
async fn test_book_failure() {
    let client = WsClient::new_public().await.unwrap();
    let params = BookParams {
        instrument_name: UNKNOWN_INSTRUMENT.to_string(),
    };
    let result = client.rpc().market_data().book(params).await;
    assert!(result.is_err(), "Expected market data book to fail");
    client.shutdown("Test complete").await.unwrap();
}
