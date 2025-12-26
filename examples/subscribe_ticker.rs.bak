use log::{Level::Info, info};
use simple_logger::init_with_level;
use thalex_rust_sdk::{models::Delay, types::ExternalEvent, ws_client::WsClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_with_level(Info).unwrap();

    let client = WsClient::new_public().await.unwrap();

    let instruments = client.get_instruments().await.unwrap();
    info!("Total Instruments: {}", instruments.len());

    let _ = client
        .subscriptions()
        .market_data()
        .ticker("BTC-PERPETUAL", Delay::Variant1000ms, |msg| {
            // Parses into a json value initally
            async move {
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
            info!(
                "Ticker update - Bid: {best_bid_price}, Ask: {best_ask_price} spread: {spread} index: {index_price} index_delta_bps: {index_delta_bps}"
            );
        }
    })
        .await;

    client.wait_for_connection().await;
    info!("Starting receive loop!");
    loop {
        match client.run_till_event().await {
            ExternalEvent::Connected => {
                client.resubscribe_all().await.ok();
            }
            ExternalEvent::Disconnected => continue,
            ExternalEvent::Exited => break,
        }
    }
    client.shutdown("Time to go!").await.unwrap();
    Ok(())
}
