use log::{Level::Info, info};
use simple_logger::init_with_level;
use thalex_rust_sdk::{
    types::{Environment, ExternalEvent},
    ws_client::WsClient,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_with_level(Info).unwrap();

    let client = WsClient::new_public(Environment::Mainnet).await.unwrap();

    let instruments = client.rpc().market_data().instruments().await.unwrap();
    info!("Total Instruments: {}", instruments.len());

    let _ = client
        .subscriptions()
        .market_data()
        .instruments(|msg| {
            // Parses into a json value initally
            async move {
                info!("Instruments update - {:?}", msg);
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
