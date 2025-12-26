use crate::{
    rpc::session_management::SessionManagementRpc, rpc::trading::TradingRpc, ws_client::WsClient,
};

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
}
