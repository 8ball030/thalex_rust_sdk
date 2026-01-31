use std::time::Duration;

use log::Level::Info;
use simple_logger::init_with_level;
use thalex_rust_sdk::{
    models::Delay,
    types::{Environment, ExternalEvent, InternalCommand},
    ws_client::WsClient,
};

#[tokio::test]
async fn test_websocket_subscription_working() {
    let client = WsClient::new_public(Environment::Testnet).await.unwrap();
    let result = client
        .subscriptions()
        .market_data()
        .ticker("BTC-PERPETUAL", Delay::Raw, |_msg| async move {
            // info!("Received ticker update: {msg:?}");
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
    let client = WsClient::new_public(Environment::Testnet).await.unwrap();
    let result = client
        .subscriptions()
        .market_data()
        .ticker("NOT_EXISTING", Delay::Raw, |_msg| async move {
            // info!("Received ticker update: {msg:?}");
        })
        .await;
    assert!(result.is_err(), "Expected subscription to fail");
    client.shutdown("Test complete").await.unwrap();
}

#[tokio::test]
async fn test_client_shutdown() {
    let client = WsClient::new_public(Environment::Testnet).await.unwrap();
    tokio::time::sleep(Duration::from_millis(200)).await;

    let result = tokio::time::timeout(Duration::from_secs(5), client.shutdown("test")).await;

    assert!(
        result.is_ok(),
        "Shutdown timed out - supervisor didn't exit"
    );
    tokio::time::sleep(Duration::from_millis(500)).await; // Let supervisor finish
}

#[tokio::test]
async fn test_ws_reconnect() {
    init_with_level(Info).unwrap();
    let client = WsClient::new_public(Environment::Testnet).await.unwrap();
    // let client = Arc::new(client);
    let disconnect_triger = client.write_tx.clone();

    let result = client
        .subscriptions()
        .market_data()
        .ticker("BTC-PERPETUAL", Delay::Raw, |_msg| async move {
            // info!("Received ticker update: {msg:?}");
        })
        .await;
    assert!(result.is_ok(), "Subscription failed: {:?}", result.err());

    client.wait_for_connection().await;

    async fn run_loop(
        client: WsClient,
        tx: tokio::sync::watch::Sender<bool>,
        recon_tx: tokio::sync::watch::Sender<bool>,
    ) {
        loop {
            match client.run_till_event().await {
                ExternalEvent::Disconnected => {
                    println!("Client disconnected, notifying test");
                    let _ = tx.send(true);
                }
                ExternalEvent::Connected => {
                    println!("Client reconnected");
                    let _ = recon_tx.send(true);
                }
                ExternalEvent::Exited => {
                    println!("Client exited");
                    break;
                }
            }
        }
    }
    let (tx, mut rx) = tokio::sync::watch::channel(false);
    let (recon_tx, mut recon_rx) = tokio::sync::watch::channel(false);
    let handle = tokio::spawn(run_loop(client, tx.clone(), recon_tx.clone()));

    disconnect_triger.send(InternalCommand::Close).unwrap();
    // Wait for initial connection
    let mut disconnected = false;
    let mut reconnected = false;
    loop {
        tokio::select! {
            _ = rx.changed() => {
                if *rx.borrow() {
                    println!("Test detected disconnection");
                    disconnected = true;
                }
            }
            _ = recon_rx.changed() => {
                if *recon_rx.borrow() {
                    println!("Test detected reconnection");
                    reconnected = true;
                }
            }
        }
        if disconnected && reconnected {
            break;
        }
    }

    handle.abort();
}
