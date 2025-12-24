use crate::ws_client::WsClient;

pub struct TradingRpc<'a> {
    pub client: &'a WsClient,
}
impl<'a> TradingRpc<'a> {
    // Trading RPC methods would go here
}
