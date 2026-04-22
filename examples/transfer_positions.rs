// Small tool to transfer positions from one account to another. Useful for testing.

use log::info;
use rust_decimal::Decimal;
use thalex_rust_sdk::{models::{InternalTransferParams, VerifyInternalTransferParamsPositionsInner}, ws_client::WsClient};

use clap::Parser;

// config for cli
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Config {
    /// The destination account to transfer positions to
    #[clap(short, long)]
    destination_account: String,
    /// The amount to transfer
    #[clap(short, long)]
    amount: Decimal,
    // position name, e.g. "BTC-PERPETUAL"
    #[clap(short, long)]
    position_name: String,
}



#[tokio::main]
async fn main() {
    simple_logger::init_with_level(log::Level::Info).unwrap();
    let config = Config::parse();
    info!("Transferring {} of {} to account {}", config.amount, config.position_name, config.destination_account);
    let client = WsClient::from_env().await.unwrap();

    let result = client
        .rpc()
        .wallet()
        .internal_transfer(
            InternalTransferParams {
                destination_account_number: config.destination_account,
                positions: Some(vec![VerifyInternalTransferParamsPositionsInner {
                    instrument_name: config.position_name,
                    amount: config.amount,
                }]),
                ..Default::default()
            }
        )
        .await;
    info!("Result: {:?}", result);
}
