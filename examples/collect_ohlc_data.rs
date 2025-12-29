use chrono::DateTime;
use thalex_rust_sdk::{
    manual_models::{
        Resolution,
        historic_data_mark::{MarkPriceData, MarkPriceHistoricalDataParams, PerpetualDataPoint},
    },
    ws_client::WsClient,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = WsClient::new_public().await.unwrap();

    // 1 day ago in unix timestamp
    let from = (chrono::Utc::now() - chrono::Duration::days(1)).timestamp() as f64;
    // now in unix timestamp
    let now = (chrono::Utc::now() - chrono::Duration::days(0)).timestamp() as f64;

    let mut all_data = fetch_mark_price_historical_data(&client, from, now, 3).await;

    loop {
        let all_stamps: Vec<f64> = all_data.iter().map(|dp| dp.0).collect();
        let first_timestamp = all_stamps
            .iter()
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .cloned()
            .unwrap_or(now);

        // Format as date time for logging
        let first_date = DateTime::from_timestamp(first_timestamp as i64, 0).unwrap();
        println!("Oldest timestamp fetched so far: {first_date}");
        let from_ts = first_timestamp - 3600.0 * 24.0 * 7.0; // go back another week

        if first_timestamp <= from {
            break;
        }
        let new_data = fetch_mark_price_historical_data(&client, from_ts, first_timestamp, 3).await;
        all_data.extend(new_data.clone());
        if new_data.is_empty() {
            println!("No more data returned, stopping fetch.");
            break;
        }
        println!(
            "Fetched {} data points, total {} oldest timestamp: {}",
            new_data.len(),
            all_data.len(),
            first_timestamp
        );
        // We sleep for 1 second to avoid rate limiting
        tokio::time::sleep(std::time::Duration::from_secs(1)).await
    }
    println!("Total data points fetched: {}", all_data.len());
    // We now have all the data we need
    // we order it by timestamp
    all_data.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    // We use csv to write the data to a new file
    let mut wtr = csv::Writer::from_path("mark_price_historical_data.csv")?;
    wtr.write_record(["timestamp", "open", "high", "low", "close"])?;

    let mut count = 0;
    for PerpetualDataPoint(time, open, high, low, close, _funding_payment, _top_of_book) in
        &all_data
    {
        wtr.write_record(&[
            time.to_string(),
            open.to_string(),
            high.to_string(),
            low.to_string(),
            close.to_string(),
        ])?;
        count += 1;
    }
    wtr.flush()?;
    println!("Wrote {count} records to mark_price_historical_data.csv");
    let all_stamps: Vec<f64> = all_data.iter().map(|dp| dp.0).collect();
    let first_timestamp = all_stamps
        .iter()
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .cloned()
        .unwrap_or(now);
    let last_timestamp = all_stamps
        .iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .cloned()
        .unwrap_or(now);
    let first_date = DateTime::from_timestamp(first_timestamp as i64, 0).unwrap();
    let last_date = DateTime::from_timestamp(last_timestamp as i64, 0).unwrap();
    println!("Data range: {first_date} to {last_date}");

    Ok(())
}
async fn fetch_mark_price_historical_data(
    client: &WsClient,
    from: f64,
    to: f64,
    max_retries: usize,
) -> Vec<PerpetualDataPoint> {
    let mut retries_left = max_retries;

    loop {
        let params = MarkPriceHistoricalDataParams {
            instrument_name: "BTC-PERPETUAL".to_string(),
            from,
            to,
            resolution: Resolution::Variant1m,
        };

        let result = client
            .rpc()
            .historical_data()
            .mark_price_historical_data(params)
            .await;

        match result {
            Ok(data) => {
                if let Some(MarkPriceData::Perpetual(perp)) = data.mark {
                    return perp;
                } else {
                    return vec![];
                }
            }
            Err(e) => {
                if retries_left == 0 {
                    println!("Max retries reached, returning empty data. Last error: {e:?}");
                    return vec![];
                } else {
                    println!(
                        "Error fetching data, retrying... ({retries_left} retries left). Error: {e:?}"
                    );
                    retries_left -= 1;
                    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                    continue;
                }
            }
        }
    }
}
