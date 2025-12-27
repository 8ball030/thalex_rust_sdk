use serial_test::serial;

use thalex_rust_sdk::{
    models::{
        DailyMarkHistoryParams, OrderHistoryParams, RequiredMarginForOrderParams, RfqHistoryParams,
        TradeHistoryParams, TransactionHistoryParams,
    },
    ws_client::WsClient,
};

const DELAY: u64 = 2000;
macro_rules! require_env {
    ($($var:expr),+ $(,)?) => {
        (
            $(
                match std::env::var($var) {
                    Ok(v) => v,
                    Err(_) => {
                        eprintln!("Skipping test: {} not set", $var);
                        return;
                    }
                }
            ),+
        )
    };
}
#[tokio::test]
#[serial]
async fn test_portfolio() {
    dotenv::dotenv().ok();
    let (_, _, _) = require_env!(
        "THALEX_PRIVATE_KEY_PATH",
        "THALEX_KEY_ID",
        "THALEX_ACCOUNT_ID"
    );
    let client = WsClient::from_env().await.unwrap();
    tokio::time::sleep(std::time::Duration::from_millis(DELAY)).await;
    let portfolio_result = client.rpc().accounting().portfolio().await;
    assert!(
        portfolio_result.is_ok(),
        "Accounting portfolio failed: {:?}",
        portfolio_result.err()
    );
    // We have to sleep a bit to avoid rate limiting in tests
    client.shutdown("Test complete").await.unwrap();
}
#[tokio::test]
#[serial]
async fn test_open_orders() {
    dotenv::dotenv().ok();
    let (_, _, _) = require_env!(
        "THALEX_PRIVATE_KEY_PATH",
        "THALEX_KEY_ID",
        "THALEX_ACCOUNT_ID"
    );
    let client = WsClient::from_env().await.unwrap();
    tokio::time::sleep(std::time::Duration::from_millis(DELAY)).await;
    let open_orders_result = client.rpc().accounting().open_orders().await;
    assert!(
        open_orders_result.is_ok(),
        "Accounting open_orders failed: {:?}",
        open_orders_result.err()
    );
    client.shutdown("Test complete").await.unwrap();
}

#[tokio::test]
#[serial]
async fn test_account_summary() {
    dotenv::dotenv().ok();
    let (_, _, _) = require_env!(
        "THALEX_PRIVATE_KEY_PATH",
        "THALEX_KEY_ID",
        "THALEX_ACCOUNT_ID"
    );
    let client = WsClient::from_env().await.unwrap();
    tokio::time::sleep(std::time::Duration::from_millis(DELAY)).await;
    let account_summary_result = client.rpc().accounting().account_summary().await;
    assert!(
        account_summary_result.is_ok(),
        "Accounting account_summary failed: {:?}",
        account_summary_result.err()
    );
    client.shutdown("Test complete").await.unwrap();
}

#[tokio::test]
#[serial]
async fn test_account_breakdown() {
    dotenv::dotenv().ok();
    let (_, _, _) = require_env!(
        "THALEX_PRIVATE_KEY_PATH",
        "THALEX_KEY_ID",
        "THALEX_ACCOUNT_ID"
    );
    let client = WsClient::from_env().await.unwrap();
    tokio::time::sleep(std::time::Duration::from_millis(DELAY)).await;
    let account_breakdown_result = client.rpc().accounting().account_breakdown().await;
    assert!(
        account_breakdown_result.is_ok(),
        "Accounting account_breakdown failed: {:?}",
        account_breakdown_result.err()
    );
    client.shutdown("Test complete").await.unwrap();
}

#[tokio::test]
#[serial]
async fn test_order_history() {
    dotenv::dotenv().ok();
    let (_, _, _) = require_env!(
        "THALEX_PRIVATE_KEY_PATH",
        "THALEX_KEY_ID",
        "THALEX_ACCOUNT_ID"
    );
    let client = WsClient::from_env().await.unwrap();
    tokio::time::sleep(std::time::Duration::from_millis(DELAY)).await;
    let params = OrderHistoryParams::default();

    let result = client.rpc().accounting().order_history(params).await;
    assert!(
        result.is_ok(),
        "Accounting order_history failed: {:?}",
        result.err()
    );
    client.shutdown("Test complete").await.unwrap();
}

#[tokio::test]
#[serial]
async fn test_trade_history() {
    dotenv::dotenv().ok();
    let (_, _, _) = require_env!(
        "THALEX_PRIVATE_KEY_PATH",
        "THALEX_KEY_ID",
        "THALEX_ACCOUNT_ID"
    );
    let client = WsClient::from_env().await.unwrap();
    tokio::time::sleep(std::time::Duration::from_millis(DELAY)).await;
    let params = TradeHistoryParams::default();

    let result = client.rpc().accounting().trade_history(params).await;
    assert!(
        result.is_ok(),
        "Accounting trade_history failed: {:?}",
        result.err()
    );
    client.shutdown("Test complete").await.unwrap();
}

#[tokio::test]
#[serial]
async fn test_daily_mark_history() {
    dotenv::dotenv().ok();
    let (_, _, _) = require_env!(
        "THALEX_PRIVATE_KEY_PATH",
        "THALEX_KEY_ID",
        "THALEX_ACCOUNT_ID"
    );
    let client = WsClient::from_env().await.unwrap();
    tokio::time::sleep(std::time::Duration::from_millis(DELAY)).await;
    let params = DailyMarkHistoryParams::default();

    let result = client.rpc().accounting().daily_mark_history(params).await;
    assert!(
        result.is_ok(),
        "Accounting daily_mark_history failed: {:?}",
        result.err()
    );
    client.shutdown("Test complete").await.unwrap();
}

#[tokio::test]
#[serial]
async fn test_transaction_history() {
    dotenv::dotenv().ok();
    let (_, _, _) = require_env!(
        "THALEX_PRIVATE_KEY_PATH",
        "THALEX_KEY_ID",
        "THALEX_ACCOUNT_ID"
    );
    let client = WsClient::from_env().await.unwrap();
    tokio::time::sleep(std::time::Duration::from_millis(DELAY)).await;
    let params = TransactionHistoryParams::default();

    let result = client.rpc().accounting().transaction_history(params).await;
    assert!(
        result.is_ok(),
        "Accounting transaction_history failed: {:?}",
        result.err()
    );
    client.shutdown("Test complete").await.unwrap();
}

#[tokio::test]
#[serial]
async fn test_rfq_history() {
    dotenv::dotenv().ok();
    let (_, _, _) = require_env!(
        "THALEX_PRIVATE_KEY_PATH",
        "THALEX_KEY_ID",
        "THALEX_ACCOUNT_ID"
    );
    let client = WsClient::from_env().await.unwrap();
    tokio::time::sleep(std::time::Duration::from_millis(DELAY)).await;
    let params = RfqHistoryParams::default();

    let result = client.rpc().accounting().rfq_history(params).await;
    assert!(
        result.is_ok(),
        "Accounting rfq_history failed: {:?}",
        result.err()
    );
    client.shutdown("Test complete").await.unwrap();
}

#[tokio::test]
#[serial]
async fn test_required_margin_for_order() {
    dotenv::dotenv().ok();
    let (_, _, _) = require_env!(
        "THALEX_PRIVATE_KEY_PATH",
        "THALEX_KEY_ID",
        "THALEX_ACCOUNT_ID"
    );
    let client = WsClient::from_env().await.unwrap();
    tokio::time::sleep(std::time::Duration::from_millis(DELAY)).await;
    let params = RequiredMarginForOrderParams {
        instrument_name: Some("BTC-PERPETUAL".to_string()),
        amount: 0.001,
        price: 85000.0,
        legs: None,
    };

    let result = client
        .rpc()
        .accounting()
        .required_margin_for_order(params)
        .await;
    assert!(
        result.is_ok(),
        "Accounting required_margin_for_order failed: {:?}",
        result.err()
    );
    client.shutdown("Test complete").await.unwrap();
}
