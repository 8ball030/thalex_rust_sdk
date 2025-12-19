use log::{Level::Info, info};
use simple_logger::init_with_level;
use thalex_rust_sdk::{models::Trade, ws_client::WsClient};

use serde::ser::{Serialize, SerializeStruct, Serializer};

pub struct TradeCsv<'a>(pub &'a Trade);

impl<'a> Serialize for TradeCsv<'a> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let t = self.0;

        // 22 columns, always emitted in the same order.
        let mut st = serializer.serialize_struct("Trade", 22)?;

        st.serialize_field("trade_type", &t.trade_type)?;
        st.serialize_field("trade_id", &t.trade_id)?;
        st.serialize_field("order_id", &t.order_id)?;
        st.serialize_field("instrument_name", &t.instrument_name)?;
        st.serialize_field("direction", &t.direction)?;
        st.serialize_field("price", &t.price)?;
        st.serialize_field("amount", &t.amount)?;
        st.serialize_field("label", &t.label)?;
        st.serialize_field("time", &t.time)?;
        st.serialize_field("position_after", &t.position_after)?;
        st.serialize_field("session_realised_after", &t.session_realised_after)?;
        st.serialize_field("position_pnl", &t.position_pnl)?;
        st.serialize_field("perpetual_funding_pnl", &t.perpetual_funding_pnl)?;
        st.serialize_field("fee", &t.fee)?;
        st.serialize_field("index", &t.index)?;
        st.serialize_field("fee_rate", &t.fee_rate)?;
        st.serialize_field("funding_mark", &t.funding_mark)?;
        st.serialize_field("liquidation_fee", &t.liquidation_fee)?;
        st.serialize_field("client_order_id", &t.client_order_id)?;
        st.serialize_field("maker_taker", &t.maker_taker)?;
        st.serialize_field("bot_id", &t.bot_id)?;
        st.serialize_field("leg_index", &t.leg_index)?;

        st.end()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_with_level(Info).unwrap();

    dotenv::dotenv().ok();
    let client = WsClient::from_env().await.unwrap();

    let mut all_trades = Vec::new();
    let trade_result = client.get_trade_history(None).await.unwrap();

    all_trades.extend(trade_result.trades.unwrap_or_default());
    let mut total_trades = all_trades.len();

    // While there is a bookmark, keep fetching more trades
    let mut bookmark = trade_result.bookmark;
    while let Some(bm) = bookmark {
        let trade_result = client.get_trade_history(Some(bm.clone())).await.unwrap();
        bookmark = trade_result.bookmark;
        let new_trades = trade_result.trades.unwrap_or_default();
        total_trades += new_trades.len();
        all_trades.extend(new_trades);
    }
    info!("Total trades collected: {total_trades}");

    // We write the trades to a csv file.
    let mut processed_trades = Vec::new();
    let filename = "all_trades.csv".to_string();
    let mut wtr = csv::WriterBuilder::new()
        .has_headers(true)
        .from_path(filename.clone())?;
    for trade in all_trades.iter() {
        wtr.serialize(TradeCsv(trade))?;
        processed_trades.push(trade.clone());
    }
    wtr.flush()?;
    info!("All trades written to {filename}");
    // we check that we processed all trades
    assert_eq!(
        all_trades.len(),
        processed_trades.len(),
        "Some trades were not processed!"
    );

    client.shutdown("Finished collecting trades").await.unwrap();
    Ok(())
}
