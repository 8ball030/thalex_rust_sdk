use crate::{rpc::trading::TradingRpc, ws_client::WsClient};

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
}
