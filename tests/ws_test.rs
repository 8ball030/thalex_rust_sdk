use log::info;
use thalex_rust_sdk::{models::Delay, ws_client::WsClient};

#[tokio::test]
async fn test_websocket_subscription_working() {
    let client = WsClient::new_public().await.unwrap();
    let result = client
        .subscriptions()
        .market_data()
        .ticker("BTC-PERPETUAL", Delay::Raw, |msg| async move {
            info!("Received ticker update: {msg:?}");
        })
        .await;
    assert!(result.is_ok(), "Subscription failed: {:?}", result.err());
    client.shutdown("Test complete").await.unwrap();
}

#[tokio::test]
async fn test_websocket_subscription_not_working() {
    let client = WsClient::new_public().await.unwrap();
    let result = client
        .subscriptions()
        .market_data()
        .ticker("NOT_EXISTING", Delay::Raw, |msg| async move {
            info!("Received ticker update: {msg:?}");
        })
        .await;
    assert!(result.is_err(), "Expected subscription to fail");
    client.shutdown("Test complete").await.unwrap();
}
