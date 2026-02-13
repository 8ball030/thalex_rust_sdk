use rust_decimal_macros::dec;
use thalex_rust_sdk::models::{
    CancelMassQuoteParams, DoubleSidedQuote, DoubleSidedQuoteB, MassQuoteParams,
    SetMmProtectionParams, SingleLevelQuote,
};
use thalex_rust_sdk::ws_client::WsClient;

mod common;

const KNOWN_GROUP: &str = "FBTCUSD";
const KNOWN_PERP: &str = "BTC-PERPETUAL";

#[tokio::test]
#[serial_test::serial(private_rpc)]
async fn test_mm_flow() {
    dotenv::dotenv().ok();
    let (_, _, _) = require_env!(
        "THALEX_PRIVATE_KEY_PATH",
        "THALEX_KEY_ID",
        "THALEX_ACCOUNT_ID"
    );
    let client = WsClient::from_env().await.unwrap();
    client.set_cancel_on_disconnect().await.unwrap();
    // Set MM protection
    let set_protection_result = client
        .rpc()
        .mm()
        .set_mm_protection(SetMmProtectionParams {
            product: KNOWN_GROUP.to_string(),
            amount: Some(dec!(0.01)),
            quote_amount: dec!(30.0),
            trade_amount: dec!(0.01),
        })
        .await;
    assert!(
        set_protection_result.is_ok(),
        "Set MM protection failed: {:?}",
        set_protection_result.err()
    );
    // Mass quote
    let mass_quote_result = client
        .rpc()
        .mm()
        .mass_quote(MassQuoteParams {
            reject_post_only: Some(true),
            post_only: Some(true),
            label: Some("test_mass_quote_flow".to_string()),
            quotes: vec![DoubleSidedQuote {
                i: KNOWN_PERP.to_string(),
                b: Some(DoubleSidedQuoteB::SingleLevelQuote(SingleLevelQuote {
                    p: dec!(1500.0),
                    a: dec!(0.001),
                })),
                a: None,
            }],
            ..Default::default()
        })
        .await;
    assert!(
        mass_quote_result.is_ok(),
        "Mass quote failed: {:?}",
        mass_quote_result.err()
    );
    // Cancel mass quote
    let cancel_mass_quote_result = client
        .rpc()
        .mm()
        .cancel_mass_quote(CancelMassQuoteParams {
            product: Some(KNOWN_GROUP.to_string()),
        })
        .await;
    assert!(
        cancel_mass_quote_result.is_ok(),
        "Cancel mass quote failed: {:?}",
        cancel_mass_quote_result.err()
    );
}
