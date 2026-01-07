use log::{Level::Info, info};
use simple_logger::init_with_level;
use thalex_rust_sdk::{types::ExternalEvent, ws_client::WsClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_with_level(Info).unwrap();
    dotenv::dotenv().ok();

    let client = WsClient::from_env().await.unwrap();
    client.wait_for_connection().await;

    let _ = client
        .subscriptions()
        .accounting()
        .account_orders(|msg| {
            async move{
            for order in msg{
                info!(
                    "Account Order Update: id={} instrument_name={:?} order_type={:?} amount={} price={:?}",
                    order.order_id, order.instrument_name, order.order_type, order.amount, order.price
                );
            }
            }
        })
        .await;

    let _ = client
        .subscriptions()
        .accounting()
        .account_portfolio(|msg| {
            async move{
            for portfolio in msg{
                info!(
                    "Account Portfolio Update: instrument_name={:?} position={:?} mark_price={:?} average_price={:?} realised_pnl={:?}",
                    portfolio.instrument_name, portfolio.position, portfolio.mark_price, portfolio.average_price, portfolio.realised_pnl
                );
            }
        }
        })
        .await;
    info!("Starting receive loop!");
    loop {
        match client.run_till_event().await {
            ExternalEvent::Connected => {
                info!("Connected!");
                client.login().await.ok();
                client.resubscribe_all().await.ok();
            }
            ExternalEvent::Disconnected => {
                info!("Disconnected, waiting for reconnect...");
                continue;
            }
            ExternalEvent::Exited => {
                info!("Client exited, stopping receive loop.");
                break;
            }
        }
    }
    client.shutdown("Time to go!").await.unwrap();
    Ok(())
}
