use log::{Level::Info, info};
use simple_logger::init_with_level;
use thalex_rust_sdk::ws_client::WsClient;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_with_level(Info).unwrap();
    dotenv::dotenv().ok();

    let client = WsClient::from_env().await.unwrap();

    sleep(tokio::time::Duration::from_secs(1)).await;
    let _ = client
        .subscriptions()
        .accounting()
        .account_orders(|msg| {
            for order in msg{
                info!(
                    "Account Order Update: id={} instrument_name={:?} order_type={:?} amount={} price={:?}",
                    order.order_id, order.instrument_name, order.order_type, order.amount, order.price
                );
            }
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
