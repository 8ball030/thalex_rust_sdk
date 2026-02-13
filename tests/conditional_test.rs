mod common;

use rust_decimal_macros::dec;
use thalex_rust_sdk::{
    models::{
        CancelConditionalOrderParams, CreateConditionalOrderParams, DirectionEnum, TargetEnum,
    },
    ws_client::WsClient,
};

#[macro_export]
macro_rules! params_private_conditional_rpc_test {
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
                    .cancel_conditional_order(CancelConditionalOrderParams {
                        order_id: result.unwrap().order_id,
                    })
                    .await
            });
            assert!(
                cancel_result.is_ok(),
                "Cancel conditional order failed: {:?}",
                cancel_result.err()
            );
        }
    };
}
no_params_private_rpc_test!(
    test_conditional_orders_cancel_all_conditional_orders,
    cancel_all_conditional_orders,
    "Conditional cancel_all_conditional_orders",
    conditional
);

no_params_private_rpc_test!(
    test_conditional_orders,
    conditional_orders,
    "Conditional conditional_orders",
    conditional
);

params_private_conditional_rpc_test!(
    test_conditional_order_create,
    CreateConditionalOrderParams {
        instrument_name: "BTC-PERPETUAL".to_string(),
        amount: dec!(0.0001),
        stop_price: dec!(30000.0),
        trailing_stop_callback_rate: Some(dec!(0.05)), // 5%
        reduce_only: Some(true),
        direction: DirectionEnum::Sell,
        target: Some(TargetEnum::Index),
        label: Some("test_order".to_string()),
        ..Default::default()
    },
    create_conditional_order,
    "Conditional create_conditional_order",
    conditional
);
