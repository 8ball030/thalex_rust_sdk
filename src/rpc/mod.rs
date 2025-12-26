use crate::{
    rpc::{
        market_data::MarketDataRpc, session_management::SessionManagementRpc, trading::TradingRpc,
    },
    ws_client::WsClient,
};

pub mod market_data;
pub mod session_management;
pub mod trading;

pub struct Rpc<'a> {
    pub client: &'a WsClient,
}
impl<'a> Rpc<'a> {
    pub fn trading(&self) -> TradingRpc<'a> {
        TradingRpc {
            client: self.client,
        }
    }

    pub fn session_management(&self) -> SessionManagementRpc<'a> {
        SessionManagementRpc {
            client: self.client,
        }
    }

    pub fn market_data(&self) -> MarketDataRpc<'a> {
        MarketDataRpc {
            client: self.client,
        }
    }
}
