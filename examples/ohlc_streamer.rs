// Simple example of streaming tickers into Candles/OHLCV
// using thalex_rust_sdk

use std::sync::Arc;

use chrono::{DateTime, Utc};
use log::{Level::Info, info};
use rust_decimal::{Decimal, prelude::FromPrimitive};
use simple_logger::init_with_level;
use thalex_rust_sdk::{models::Delay, models::Ticker, types::ExternalEvent, ws_client::WsClient};
use tokio::sync::Mutex;

#[derive(Debug, Clone, Default)]
pub struct Candle {
    pub timestamp: DateTime<Utc>,
    pub open: Decimal,
    pub high: Decimal,
    pub low: Decimal,
    pub close: Decimal,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_with_level(Info).unwrap();

    let client = WsClient::new_public().await.unwrap();

    let interval = 60; // 1 minute candles

    let now_ms: i64 = Utc::now().timestamp_millis();
    let now_dt = from_time_stamp_to_date_time(now_ms);

    info!("Starting interval timestamp: {now_ms} ({now_dt})");

    let interval_end_ms = round_timestamp_to_interval(now_ms, interval);
    let interval_end_dt = from_time_stamp_to_date_time(interval_end_ms);

    let current_candle = Arc::new(Mutex::new(Candle {
        timestamp: interval_end_dt,
        ..Default::default()
    }));

    let _ = client
        .subscriptions()
        .market_data()
        .ticker("BTC-PERPETUAL", Delay::Variant100ms, move |msg| {
            let candle = current_candle.clone();
            async move { on_ticker(msg, candle, interval).await }
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

async fn on_ticker(msg: Ticker, candle: Arc<Mutex<Candle>>, interval: i64) {
    if msg.best_ask_price.is_none() || msg.best_bid_price.is_none() {
        return;
    }
    let mark_price: Decimal = Decimal::from_f64(msg.mark_price.unwrap()).unwrap();
    let timestamp = (msg.mark_timestamp.unwrap() * 1000.0) as i64;
    let mut current_candle = candle.lock().await;

    if Some(current_candle.high) < Some(mark_price) {
        current_candle.high = mark_price;
    }
    if Some(current_candle.low) > Some(mark_price) {
        current_candle.low = mark_price;
    }
    current_candle.close = mark_price;

    if timestamp >= current_candle.timestamp.timestamp() * 1000 {
        current_candle.close = mark_price;
        // Emit candle
        info!(
            "Candle@{}: T: {:.0} O: {:.2}, H: {:.2}, L: {:.2}, C: {:.2}",
            current_candle.timestamp,
            from_datetime_to_timestamp(current_candle.timestamp) / 1000,
            current_candle.open,
            current_candle.high,
            current_candle.low,
            current_candle.close,
        );
        // Reset candle
        let interval_ending_timestamp = round_timestamp_to_interval(timestamp, interval);
        *current_candle = Candle {
            open: mark_price,
            high: mark_price,
            low: mark_price,
            close: mark_price,
            timestamp: from_time_stamp_to_date_time(interval_ending_timestamp),
        };
    }
}

fn from_time_stamp_to_date_time(timestamp: i64) -> DateTime<Utc> {
    let naive = DateTime::from_timestamp_millis(timestamp);
    naive.unwrap().with_timezone(&Utc)
}

fn from_datetime_to_timestamp(datetime: DateTime<Utc>) -> i64 {
    datetime.timestamp_millis()
}

fn round_timestamp_to_interval(timestamp_ms: i64, interval_seconds: i64) -> i64 {
    let interval_ms = interval_seconds * 1000;
    ((timestamp_ms as f64 / interval_ms as f64).ceil() * interval_ms as f64) as i64
}
