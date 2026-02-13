mod common;

use rust_decimal_macros::dec;
use thalex_rust_sdk::{
    manual_models::error_code::ErrorCode,
    models::{CancelParams, InsertParams},
    ws_client::WsClient,
};

macro_rules! params_private_trading_rpc_test {
    ($name:ident, $params:expr, $method:ident, $label:literal, $namespace:ident) => {
        #[tokio::test]
        #[serial_test::serial(private_rpc)]
        async fn $name() {
            let result =
                with_private_client!(client, { client.rpc().$namespace().$method($params).await });
            assert!(result.is_ok(), "{} failed: {:?}", $label, result.err());
            let cancel_result = with_private_client!(client, {
                client
                    .rpc()
                    .$namespace()
                    .cancel(CancelParams {
                        order_id: Some(result.unwrap().order_id),
                        ..Default::default()
                    })
                    .await
            });
            assert!(
                cancel_result.is_ok(),
                "Cancel order failed: {:?}",
                cancel_result.err()
            );
        }
    };
}

params_private_trading_rpc_test!(
    test_limit_order_success,
    InsertParams {
        instrument_name: Some("BTC-PERPETUAL".to_string()),
        amount: dec!(0.001),
        price: Some(dec!(10000.0)),
        legs: None,
        ..Default::default()
    },
    insert,
    "Trading insert limit order successfully",
    trading
);

#[tokio::test]
#[serial_test::serial(private_rpc)]
async fn test_limit_order_failure() {
    let result = with_private_client!(client, {
        client
            .rpc()
            .trading()
            .insert(InsertParams {
                instrument_name: Some("BTC-PERPETUAL".to_string()),
                amount: dec!(0.00100001),
                price: Some(dec!(10000.92)),
                legs: None,
                ..Default::default()
            })
            .await
    });
    assert!(
        result.is_err(),
        "Expected error but got success: {:?}",
        result.ok()
    );
    let code = result.unwrap_err().error.unwrap().code;
    match code {
        ErrorCode::PriceNotAlignedWithTick => (),
        _ => panic!("Expected PriceNotAlignedWithTick error code, got {code:?}"),
    }
}
