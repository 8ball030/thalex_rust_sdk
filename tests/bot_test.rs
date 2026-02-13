mod common;
use rust_decimal_macros::dec;
use thalex_rust_sdk::{
    models::{CreateBotParams, DHedge1},
    ws_client::WsClient,
};

no_params_private_rpc_test!(test_bot_get_bots, bots, "Bot get_bot", bot);

fn get_test_hedge_strategy() -> DHedge1 {
    DHedge1 {
        strategy: "dhedge".to_string(),
        period: dec!(3600.0),
        instrument_name: "ETH-PERPETUAL".to_string(),
        ..Default::default()
    }
}

params_private_rpc_test!(
    test_bot_create_dhedge_bot,
    CreateBotParams::DHedge1(get_test_hedge_strategy()),
    create_bot,
    "Bot create dhedge bot",
    bot
);
