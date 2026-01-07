use crate::{
    models::{
        LoginParams, LoginResponse, LoginRpcResult, RpcErrorResponse, SetCancelOnDisconnectParams,
        SetCancelOnDisconnectResponse, SetCancelOnDisconnectRpcResult,
    },
    ws_client::WsClient,
};

pub struct SessionManagementRpc<'a> {
    pub client: &'a WsClient,
}
impl<'a> SessionManagementRpc<'a> {
    /// Login
    /// returns: LoginRpcResult
    pub async fn login(&self, params: LoginParams) -> Result<LoginRpcResult, RpcErrorResponse> {
        let result: LoginResponse = self
            .client
            .send_rpc(
                "public/login",
                serde_json::to_value(params).expect("Failed to serialize params"),
            )
            .await
            .expect("Failed to send RPC request");
        match result {
            LoginResponse::LoginResult(res) => Ok(res.result),
            LoginResponse::RpcErrorResponse(err) => Err(err),
        }
    }

    /// Set cancel on disconnect
    /// returns: SetCancelOnDisconnectRpcResult
    pub async fn set_cancel_on_disconnect(
        &self,
        params: SetCancelOnDisconnectParams,
    ) -> Result<SetCancelOnDisconnectRpcResult, RpcErrorResponse> {
        let result: SetCancelOnDisconnectResponse = self
            .client
            .send_rpc(
                "private/set_cancel_on_disconnect",
                serde_json::to_value(params).expect("Failed to serialize params"),
            )
            .await
            .expect("Failed to send RPC request");
        match result {
            SetCancelOnDisconnectResponse::SetCancelOnDisconnectResult(res) => Ok(res.result),
            SetCancelOnDisconnectResponse::RpcErrorResponse(err) => Err(err),
        }
    }
}
