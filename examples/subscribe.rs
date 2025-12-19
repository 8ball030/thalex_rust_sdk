use log::{Level::Info, info};
use simple_logger::init_with_level;
use thalex_rust_sdk::{models::Delay, ws_client::WsClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_with_level(Info).unwrap();

    let client = WsClient::connect_default().await.unwrap();

    let instruments = client.get_instruments().await.unwrap();
    info!("Total Instruments: {}", instruments.len());

    let _ = client
        .subscriptions()
        .ticker("BTC-PERPETUAL", Delay::Raw, |msg| {
            // Parses into a json value initally
            let best_bid_price: f64 = msg.best_bid_price.unwrap();
            let best_ask_price: f64 = msg.best_ask_price.unwrap();
            let index_price = msg.index.unwrap();

            // Check if all non-optional fields are present
            let spread = best_ask_price - best_bid_price;

            let index_delta = msg.mark_price.unwrap() - index_price;
            let index_delta_bps = if index_price != 0.0 {
                (index_delta / index_price) * 10000.0
            } else {
                0.0
            };
            let spread_bps = if best_bid_price != 0.0 {
                (spread / best_bid_price) * 10000.0
            } else {
                0.0
            };
            info!(
                "Ticker update - Bid: {best_bid_price}, Ask: {best_ask_price} spread: {spread} spread_bps: {spread_bps} index: {index_price} index_delta_bps: {index_delta_bps}"
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
