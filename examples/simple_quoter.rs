use std::sync::Arc;

use log::{Level::Info, info};
use simple_logger::init_with_level;
use thalex_rust_sdk::{
    models::{
        AmendParams, CancelParams, Delay, DirectionEnum, InsertParams, OrderStatus, OrderTypeEnum,
        SetCancelOnDisconnectParams, StatusEnum,
    },
    types::ExternalEvent,
    ws_client::WsClient,
};
use tokio::sync::Mutex;

const MARKET_NAME: &str = "BTC-PERPETUAL";
const ORDER_SIZE: f64 = 0.0001;
const PRICE_TOLERANCE_MIN_BPS: f64 = 1.0;
const PRICE_TOLERANCE_MAX_BPS: f64 = 5.0;
const ORDER_OFFSET_BPS: f64 = (PRICE_TOLERANCE_MAX_BPS + PRICE_TOLERANCE_MIN_BPS) / 2.0;
const MAX_POSITION_SIZE: f64 = 0.001;

struct StrategyState {
    bid_order: Option<OrderStatus>,
    ask_order: Option<OrderStatus>,
    position_size: f64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_with_level(Info).unwrap();

    dotenv::dotenv().ok();
    let client = WsClient::from_env().await.unwrap();
    let _ = client.set_cancel_on_disconnect().await;
    let position = client
        .rpc()
        .accounting()
        .portfolio()
        .await
        .expect("Failed to fetch portfolio")
        .into_iter()
        .find(|p| p.instrument_name.as_deref() == Some(MARKET_NAME));

    let state = StrategyState {
        bid_order: None,
        ask_order: None,
        position_size: position.map_or(0.0, |p| p.position.unwrap_or(0.0)),
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

                let raw_bid_price = best_bid_price * (1.0 - ORDER_OFFSET_BPS / 10000.0);
                let bid_price = client
                    .round_price_to_ticks(raw_bid_price, MARKET_NAME)
                    .await
                    .unwrap();

                let raw_ask_price = best_ask_price * (1.0 + ORDER_OFFSET_BPS / 10000.0);
                let ask_price = client
                    .round_price_to_ticks(raw_ask_price, MARKET_NAME)
                    .await
                    .unwrap();
                // Check if we have active orders
                let mut state = state.lock().await;
                if state.bid_order.is_none() && state.position_size + ORDER_SIZE < MAX_POSITION_SIZE
                {
                    let bid_order = client
                        .rpc()
                        .trading()
                        .insert(InsertParams {
                            direction: DirectionEnum::Buy,
                            amount: ORDER_SIZE,
                            price: Some(bid_price),
                            instrument_name: Some(MARKET_NAME.to_string()),
                            order_type: Some(OrderTypeEnum::Limit),
                            post_only: Some(true),
                            reject_post_only: Some(true),
                            ..Default::default()
                        })
                        .await
                        .unwrap();
                    info!("Placed bid order: {bid_price:?}");
                    state.bid_order = Some(bid_order);
                }
                if state.ask_order.is_none()
                    && state.position_size - ORDER_SIZE > -MAX_POSITION_SIZE
                {
                    let ask_order = client
                        .rpc()
                        .trading()
                        .insert(InsertParams {
                            direction: DirectionEnum::Sell,
                            amount: ORDER_SIZE,
                            price: Some(ask_price),
                            instrument_name: Some(MARKET_NAME.to_string()),
                            order_type: Some(OrderTypeEnum::Limit),
                            post_only: Some(true),
                            reject_post_only: Some(true),
                            ..Default::default()
                        })
                        .await
                        .unwrap();
                    info!("Placed ask order: {ask_price:?}");
                    state.ask_order = Some(ask_order);
                }

                // We check if we need to do updates on existing orders
                if state.bid_order.is_some() && state.position_size + ORDER_SIZE < MAX_POSITION_SIZE
                {
                    let bid_order = state.bid_order.as_ref().unwrap();
                    let price_diff_bps = ((bid_price - bid_order.price.unwrap())
                        / bid_order.price.unwrap())
                        * 10000.0;
                    if price_diff_bps.abs() > PRICE_TOLERANCE_MIN_BPS
                        || price_diff_bps.abs() > PRICE_TOLERANCE_MAX_BPS
                    {
                        let updated_bid_order = client
                            .rpc()
                            .trading()
                            .amend(AmendParams {
                                order_id: Some(bid_order.order_id.clone()),
                                amount: ORDER_SIZE,
                                price: bid_price,
                                ..Default::default()
                            })
                            .await
                            .unwrap();
                        info!(
                            "Modified bid order: {:?} to new price: {:?}",
                            bid_order.order_id, bid_price
                        );
                        state.bid_order = Some(updated_bid_order);
                    }
                }
                if state.ask_order.is_some()
                    && state.position_size - ORDER_SIZE > -MAX_POSITION_SIZE
                {
                    let ask_order = state.ask_order.as_ref().unwrap();
                    let price_diff_bps = ((ask_price - ask_order.price.unwrap())
                        / ask_order.price.unwrap())
                        * 10000.0;
                    if price_diff_bps.abs() > PRICE_TOLERANCE_MIN_BPS
                        || price_diff_bps.abs() > PRICE_TOLERANCE_MAX_BPS
                    {
                        let updated_ask_order = client
                            .rpc()
                            .trading()
                            .amend(AmendParams {
                                order_id: Some(ask_order.order_id.clone()),
                                price: ask_price,
                                amount: ORDER_SIZE,
                                ..Default::default()
                            })
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
                        StatusEnum::Filled | StatusEnum::Cancelled => {
                            let mut state = state.lock().await;
                            info!("Order {} is no longer active.", order.order_id);
                            info!("Removing order from strategy state.");
                            if let Some(bid_order) = &state.bid_order
                                && bid_order.order_id == order.order_id {
                                    state.bid_order = None;
                                }
                            if let Some(ask_order) = &state.ask_order
                                && ask_order.order_id == order.order_id {
                                    state.ask_order = None;
                                }
                        }
                        _ => { }
                    }
                }
            }
        })
        .await;

    let client_for_callback = client.clone();
    let state_for_callback = state.clone();
    let _ = client
         .subscriptions()
        .accounting()
        .account_portfolio(move |msg| {
            let state = Arc::clone(&state_for_callback);
            let client = client_for_callback.clone();
            async move {
                for portfolio in msg {
                    if Some(MARKET_NAME.to_string()) != portfolio.instrument_name {
                        continue;
                    }
                    let mut state = state.lock().await;
                    state.position_size = portfolio.position.unwrap_or(0.0);
                    info!(
                        "Portfolio Update: instrument_name={:?} position={:?} mark_price={:?} average_price={:?} realised_pnl={:?}",
                        portfolio.instrument_name, portfolio.position, portfolio.mark_price, portfolio.average_price, portfolio.realised_pnl
                    );
                    // If we are over max position size, cancel orders
                    if state.position_size + ORDER_SIZE >= MAX_POSITION_SIZE {
                        info!("Position size {} exceeds max {}, cancelling orders.", state.position_size, MAX_POSITION_SIZE);
                        if let Some(bid_order) = &state.bid_order {
                            let _ = client.rpc().trading().cancel(CancelParams { order_id: Some(bid_order.order_id.clone()), ..Default::default() }).await;
                            info!("Cancelled bid order: {}", bid_order.order_id);
                            state.bid_order = None;
                        }
                    }
                    if state.position_size - ORDER_SIZE <= -MAX_POSITION_SIZE {
                        info!("Position size {} exceeds max {}, cancelling orders.", state.position_size, MAX_POSITION_SIZE);
                        if let Some(ask_order) = &state.ask_order {
                            let _ = client.rpc().trading().cancel(CancelParams { order_id: Some(ask_order.order_id.clone()), ..Default::default() }).await;
                            info!("Cancelled ask order: {}", ask_order.order_id);
                            state.ask_order = None;
                        }
                    }
                }
            }
        })
        .await;

    client.wait_for_connection().await;
    info!("Starting receive loop!");
    loop {
        match client.run_till_event().await {
            ExternalEvent::Connected => {
                client.login().await.ok();
                client
                    .rpc()
                    .session_management()
                    .set_cancel_on_disconnect(SetCancelOnDisconnectParams { timeout_secs: 6 })
                    .await
                    .ok();
                client.resubscribe_all().await.ok();
                state.lock().await.bid_order = None;
                state.lock().await.ask_order = None;
            }
            ExternalEvent::Disconnected => continue,
            ExternalEvent::Exited => break,
        }
    }
    client.shutdown("Time to go!").await.unwrap();
    Ok(())
}
