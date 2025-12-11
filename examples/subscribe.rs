use std::sync::Arc;

use log::Level::Info;
use simple_logger::init_with_level;
use thalex_rust_sdk::ws_client::WsClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_with_level(Info).unwrap();
    let client = Arc::new(WsClient::new());
    client.connect().await?;

    // Spawn background task
    let client_clone = Arc::clone(&client);
    let handle = tokio::spawn(async move {
        if let Err(e) = client_clone.run_forever().await {
            eprintln!("WebSocket error: {e}");
        }
    });

    // Subscribe to ticker
    // Make RPC call to get all instruments.
    let response = client
        .call_rpc("public/instruments", serde_json::json!({}))
        .await?;
    println!("Response: {response}");

    client
        .subscribe("ticker.BTC-PERPETUAL.100ms", |msg| {
            println!("BTC: {msg}");
        })
        .await?;

    client.disconnect().await?;
    handle.abort();

    Ok(())
}
