use serial_test::serial;

use thalex_rust_sdk::{
    models::{
        DailyMarkHistoryParams, OrderHistoryParams, RequiredMarginForOrderParams, RfqHistoryParams,
        TradeHistoryParams, TransactionHistoryParams,
    },
    ws_client::WsClient,
};

const DELAY: u64 = 2000;

/// Ensures required env vars are present or skips the test.
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

/// Common test harness:
/// - loads dotenv
/// - checks env
/// - creates client
/// - waits to avoid rate limits
/// - executes body
/// - shuts down client
macro_rules! with_client {
    ($client:ident, $body:expr) => {{
        dotenv::dotenv().ok();

        let (_, _, _) = require_env!(
            "THALEX_PRIVATE_KEY_PATH",
            "THALEX_KEY_ID",
            "THALEX_ACCOUNT_ID"
        );

        let $client = WsClient::from_env().await.unwrap();
        tokio::time::sleep(std::time::Duration::from_millis(DELAY)).await;

        let result = { $body };

        $client.shutdown("Test complete").await.unwrap();
        result
    }};
}

/// Macro for simple accounting endpoint tests without parameters.
macro_rules! accounting_test {
    ($name:ident, $method:ident, $label:literal) => {
        #[tokio::test]
        #[serial]
        async fn $name() {
            let result = with_client!(client, { client.rpc().accounting().$method().await });

            assert!(result.is_ok(), "{} failed: {:?}", $label, result.err());
        }
    };
}

/// Macro for parameterized accounting tests.
macro_rules! accounting_test_with_params {
    ($name:ident, $params:expr, $method:ident, $params_ty:ty, $label:literal) => {
        #[tokio::test]
        #[serial]
        async fn $name() {
            let params: $params_ty = $params;

            let result = with_client!(client, { client.rpc().accounting().$method(params).await });

            assert!(result.is_ok(), "{} failed: {:?}", $label, result.err());
        }
    };
}

/* ---------- Simple accounting endpoints ---------- */

accounting_test!(test_portfolio, portfolio, "Accounting portfolio");
accounting_test!(test_open_orders, open_orders, "Accounting open_orders");
accounting_test!(
    test_account_summary,
    account_summary,
    "Accounting account_summary"
);
accounting_test!(
    test_account_breakdown,
    account_breakdown,
    "Accounting account_breakdown"
);

/* ---------- Parameterized endpoints ---------- */

accounting_test_with_params!(
    test_order_history,
    OrderHistoryParams::default(),
    order_history,
    OrderHistoryParams,
    "Accounting order_history"
);

accounting_test_with_params!(
    test_trade_history,
    TradeHistoryParams::default(),
    trade_history,
    TradeHistoryParams,
    "Accounting trade_history"
);

accounting_test_with_params!(
    test_daily_mark_history,
    DailyMarkHistoryParams::default(),
    daily_mark_history,
    DailyMarkHistoryParams,
    "Accounting daily_mark_history"
);

accounting_test_with_params!(
    test_transaction_history,
    TransactionHistoryParams::default(),
    transaction_history,
    TransactionHistoryParams,
    "Accounting transaction_history"
);

accounting_test_with_params!(
    test_rfq_history,
    RfqHistoryParams::default(),
    rfq_history,
    RfqHistoryParams,
    "Accounting rfq_history"
);

accounting_test_with_params!(
    test_required_margin_for_order,
    RequiredMarginForOrderParams {
        instrument_name: Some("BTC-PERPETUAL".to_string()),
        amount: 0.001,
        price: 85_000.0,
        legs: None,
    },
    required_margin_for_order,
    RequiredMarginForOrderParams,
    "Accounting required_margin_for_order"
);
