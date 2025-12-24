#![allow(clippy::too_many_arguments)]
use serde_json::Value;

use crate::ws_client::WsClient;
use crate::{
    models::{
        OrderStatus,
        order_status::{Direction, OrderType},
    },
    types::Error,
    utils::round_to_ticks,
};

pub struct TradingRpc<'a> {
    pub client: &'a WsClient,
}
impl<'a> TradingRpc<'a> {
    pub async fn insert_order(
        &self,
        instrument_name: &str,
        amount: f64,
        price: f64,
        direction: Direction,
        order_type: OrderType,
        post_only: bool,
        reject_post_only: bool,
    ) -> Result<OrderStatus, Error> {
        let instrument = self
            .client
            .check_and_refresh_instrument_cache(instrument_name)
            .await?;
        let tick_size = instrument.tick_size.unwrap();

        let result: Value = self
            .client
            .send_rpc(
                "private/insert",
                serde_json::json!({
                    "instrument_name": instrument_name,
                    "amount": amount,
                    "price": round_to_ticks(price, tick_size),
                    "direction": direction,
                    "order_type": order_type,
                    "post_only": post_only,
                    "reject_post_only": reject_post_only

                }),
            )
            .await?;

        let order_status: OrderStatus = serde_json::from_value(result)?;
        Ok(order_status)
    }

    pub async fn amend_order(
        &self,
        order_id: String,
        instrument_name: &str,
        amount: f64,
        price: f64,
    ) -> Result<OrderStatus, Error> {
        let instrument = self
            .client
            .check_and_refresh_instrument_cache(instrument_name)
            .await?;
        let tick_size = instrument.tick_size.unwrap();
        let result: Value = self
            .client
            .send_rpc(
                "private/amend",
                serde_json::json!({
                    "order_id": order_id,
                    "amount": amount,
                    "price": round_to_ticks(price, tick_size)
                }),
            )
            .await?;

        let order_status: OrderStatus = serde_json::from_value(result)?;
        Ok(order_status)
    }
    pub async fn cancel_order(&self, order_id: String) -> Result<OrderStatus, Error> {
        let result: Value = self
            .client
            .send_rpc(
                "private/cancel",
                serde_json::json!({
                    "order_id": order_id
                }),
            )
            .await?;

        let order_status: OrderStatus = serde_json::from_value(result)?;
        Ok(order_status)
    }
    pub async fn cancel_all_orders(&self) -> Result<Vec<OrderStatus>, Error> {
        let result: Value = self
            .client
            .send_rpc("private/cancel_all", serde_json::json!({}))
            .await?;

        let orders_status: Vec<OrderStatus> = serde_json::from_value(result)?;
        Ok(orders_status)
    }
    pub async fn cancel_session(&self) -> Result<(), Error> {
        let _ = self
            .client
            .send_rpc::<Value>("private/cancel_session", serde_json::json!({}))
            .await;
        Ok(())
    }
}
