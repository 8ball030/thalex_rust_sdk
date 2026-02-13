mod common;

use rust_decimal_macros::dec;
use thalex_rust_sdk::{
    models::{
        DailyMarkHistoryParams, OrderHistoryParams, RequiredMarginForOrderParams, RfqHistoryParams,
        TradeHistoryParams, TransactionHistoryParams,
    },
    ws_client::WsClient,
};

no_params_private_rpc_test!(
    test_portfolio,
    portfolio,
    "Accounting portfolio",
    accounting
);
no_params_private_rpc_test!(
    test_open_orders,
    open_orders,
    "Accounting open_orders",
    accounting
);
no_params_private_rpc_test!(
    test_account_summary,
    account_summary,
    "Accounting account_summary",
    accounting
);
no_params_private_rpc_test!(
    test_account_breakdown,
    account_breakdown,
    "Accounting account_breakdown",
    accounting
);

params_private_rpc_test!(
    test_order_history,
    OrderHistoryParams::default(),
    order_history,
    "Accounting order_history",
    accounting
);

params_private_rpc_test!(
    test_trade_history,
    TradeHistoryParams::default(),
    trade_history,
    "Accounting trade_history",
    accounting
);

params_private_rpc_test!(
    test_daily_mark_history,
    DailyMarkHistoryParams::default(),
    daily_mark_history,
    "Accounting daily_mark_history",
    accounting
);

params_private_rpc_test!(
    test_transaction_history,
    TransactionHistoryParams::default(),
    transaction_history,
    "Accounting transaction_history",
    accounting
);

params_private_rpc_test!(
    test_rfq_history,
    RfqHistoryParams::default(),
    rfq_history,
    "Accounting rfq_history",
    accounting
);

params_private_rpc_test!(
    test_required_margin_for_order,
    RequiredMarginForOrderParams {
        instrument_name: Some("BTC-PERPETUAL".to_string()),
        amount: dec!(0.001),
        price: dec!(85_000.0),
        legs: None,
    },
    required_margin_for_order,
    "Accounting required_margin_for_order",
    accounting
);
