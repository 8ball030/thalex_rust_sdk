use crate::{
    rpc::{
        accounting::AccountingRpc, conditional::ConditionalRpc, historical_data::HistoricalDataRpc,
        market_data::MarketDataRpc, mm::MmRpc, session_management::SessionManagementRpc,
        trading::TradingRpc,
    },
    ws_client::WsClient,
};

pub mod accounting;
pub mod conditional;
pub mod historical_data;
pub mod market_data;
pub mod mm;
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

    pub fn accounting(&self) -> accounting::AccountingRpc<'a> {
        AccountingRpc {
            client: self.client,
        }
    }

    pub fn conditional(&self) -> ConditionalRpc<'a> {
        ConditionalRpc {
            client: self.client,
        }
    }

    pub fn historical_data(&self) -> HistoricalDataRpc<'a> {
        HistoricalDataRpc {
            client: self.client,
        }
    }

    pub fn mm(&self) -> MmRpc<'a> {
        MmRpc {
            client: self.client,
        }
    }
}
