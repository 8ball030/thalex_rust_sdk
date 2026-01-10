use std::time::Duration;

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
    let channel = result.unwrap();
    // unsubscribe after some time
    tokio::time::sleep(Duration::from_secs(5)).await;

    let unsubscribe_result = client.unsubscribe(&channel).await;
    assert!(
        unsubscribe_result.is_ok(),
        "Unsubscribe failed: {:?}",
        unsubscribe_result.err()
    );
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

#[tokio::test]
async fn test_client_shutdown() {
    let client = WsClient::new_public().await.unwrap();
    tokio::time::sleep(Duration::from_millis(200)).await;

    let result = tokio::time::timeout(Duration::from_secs(5), client.shutdown("test")).await;

    assert!(
        result.is_ok(),
        "Shutdown timed out - supervisor didn't exit"
    );
    tokio::time::sleep(Duration::from_millis(500)).await; // Let supervisor finish
}
