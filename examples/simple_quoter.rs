use std::sync::Arc;

use log::{Level::Info, info};
use simple_logger::init_with_level;
use thalex_rust_sdk::{
    models::{
        Delay, OrderStatus,
        order_status::{Direction, OrderType, Status},
    },
    ws_client::WsClient,
};
use tokio::sync::Mutex;

const MARKET_NAME: &str = "ETH-PERPETUAL";
const ORDER_SIZE: f64 = 0.25;
const PRICE_TOLERANCE_MIN_BPS: f64 = 0.5;
const PRICE_TOLERANCE_MAX_BPS: f64 = 1.0;
const ORDER_OFFSET_BPS: f64 = (PRICE_TOLERANCE_MAX_BPS + PRICE_TOLERANCE_MIN_BPS) / 2.0;

struct StrategyState {
    bid_order: Option<OrderStatus>,
    ask_order: Option<OrderStatus>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_with_level(Info).unwrap();

    dotenv::dotenv().ok();
    let client = WsClient::from_env().await.unwrap();
    let _ = client.set_cancel_on_disconnect().await;

    let state = StrategyState {
        bid_order: None,
        ask_order: None,
    };

    // We make a mutex to allow mutable access inside the closure

    let client = Arc::new(client);
    let state = Arc::new(Mutex::new(state));

    let client_for_callback = client.clone();
    let state_for_callback = state.clone();

    let _ = client
        .subscriptions()
        .market_data()
        .ticker(MARKET_NAME, Delay::Raw, move |msg| {
            let state = Arc::clone(&state_for_callback);
            let client = Arc::clone(&client_for_callback);

            async move {
                let best_bid_price: f64 = msg.best_bid_price.unwrap();
                let best_ask_price: f64 = msg.best_ask_price.unwrap();

                // Check if we have active orders
                let mut state = state.lock().await;
                if state.bid_order.is_none() {
                    let bid_price = best_bid_price * (1.0 - ORDER_OFFSET_BPS / 10000.0);
                    let bid_order = client
                        .insert_order(
                            MARKET_NAME,
                            ORDER_SIZE,
                            bid_price,
                            Direction::Buy,
                            OrderType::Limit,
                        )
                        .await
                        .unwrap();
                    info!("Placed bid order: {bid_price:?}");
                    state.bid_order = Some(bid_order);
                }
                if state.ask_order.is_none() {
                    let ask_price = best_ask_price * (1.0 + ORDER_OFFSET_BPS / 10000.0);
                    let ask_order = client
                        .insert_order(
                            MARKET_NAME,
                            ORDER_SIZE,
                            ask_price,
                            Direction::Sell,
                            OrderType::Limit,
                        )
                        .await
                        .unwrap();
                    info!("Placed ask order: {ask_price:?}");
                    state.ask_order = Some(ask_order);
                }

                // We check if we need to do updates on existing orders
                if state.bid_order.is_some() {
                    let bid_price = best_bid_price * (1.0 - ORDER_OFFSET_BPS / 10000.0);
                    let bid_order = state.bid_order.as_ref().unwrap();
                    let price_diff_bps = ((bid_price - bid_order.price.unwrap())
                        / bid_order.price.unwrap())
                        * 10000.0;
                    if price_diff_bps.abs() > PRICE_TOLERANCE_MIN_BPS
                        || price_diff_bps.abs() > PRICE_TOLERANCE_MAX_BPS
                    {
                        let updated_bid_order = client
                            .amend_order(
                                bid_order.order_id.clone(),
                                MARKET_NAME,
                                ORDER_SIZE,
                                bid_price,
                            )
                            .await
                            .unwrap();
                        info!(
                            "Modified bid order: {:?} to new price: {:?}",
                            bid_order.order_id, bid_price
                        );
                        state.bid_order = Some(updated_bid_order);
                    }
                }
                if state.ask_order.is_some() {
                    let ask_price = best_ask_price * (1.0 + ORDER_OFFSET_BPS / 10000.0);
                    let ask_order = state.ask_order.as_ref().unwrap();
                    let price_diff_bps = ((ask_price - ask_order.price.unwrap())
                        / ask_order.price.unwrap())
                        * 10000.0;
                    if price_diff_bps.abs() > PRICE_TOLERANCE_MIN_BPS
                        || price_diff_bps.abs() > PRICE_TOLERANCE_MAX_BPS
                    {
                        let updated_ask_order = client
                            .amend_order(
                                ask_order.order_id.clone(),
                                MARKET_NAME,
                                ORDER_SIZE,
                                ask_price,
                            )
                            .await
                            .unwrap();
                        info!(
                            "Modified ask order: {:?} to new price: {:?}",
                            ask_order.order_id, ask_price
                        );
                        state.ask_order = Some(updated_ask_order);
                    }
                }
            }
        })
        .await;

    let state_for_orders = state.clone();

    let _ = client
        .subscriptions()
        .accounting()
        .session_orders(move |msg| {
            let state = Arc::clone(&state_for_orders);
            async move {
                for order in msg {
                    info!(
                        "Order Update: id={} instrument_name={:?} order_type={:?} amount={} price={:?} status={:?}",
                        order.order_id, order.instrument_name, order.order_type, order.amount, order.price, order.status
                    );
                    match order.status {
                        Status::Filled | Status::Cancelled => {
                            let mut state = state.lock().await;
                            info!("Order {} is no longer active.", order.order_id);
                            info!("Removing order from strategy state.");
                            if let Some(bid_order) = &state.bid_order {
                                if bid_order.order_id == order.order_id {
                                    state.bid_order = None;
                                }}
                            if let Some(ask_order) = &state.ask_order {
                                if ask_order.order_id == order.order_id {
                                    state.ask_order = None;
                                }}
                        }
                        _ => { }
                    }
                }
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

    // client.shutdown("Time to go!").await.unwrap();
    Ok(())
}
