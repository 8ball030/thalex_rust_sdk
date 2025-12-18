use log::{Level::Info, info};
use serde_json::json;
use simple_logger::init_with_level;
use thalex_rust_sdk::ws_client::WsClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_with_level(Info).unwrap();

    let client = WsClient::connect_default().await.unwrap();

    let instruments = client
        .get_instruments("public/instruments", json!({}))
        .await
        .unwrap();
    info!("Instruments: {instruments:#?}");

    let _ = client
        .subscribe("ticker.BTC-PERPETUAL.raw", |msg| {
            // Parses into a json value initally
            let best_bid_price: f64 = msg.best_bid_price.unwrap_or_default();
            let best_ask_price: f64 = msg.best_ask_price.unwrap_or_default();
            let spread = best_ask_price - best_bid_price;
            let spread_bps = if best_bid_price != 0.0 {
                (spread / best_bid_price) * 10000.0
            } else {
                0.0
            };
            info!(
                "Ticker update - Best Bid: {best_bid_price}, Best Ask: {best_ask_price} spread: {spread} spread_bps: {spread_bps}"
            );
        })
        .await;

    loop {
        // Catch ctrl-c to exit
        tokio::select! {
            _ = tokio::time::sleep(tokio::time::Duration::from_secs(1)) => {}
            _ = tokio::signal::ctrl_c() => {
                println!("Ctrl-C received, shutting down");
                break;
            }
        }
    }

    client.shutdown("Time to go!").await.unwrap();
    Ok(())
}
