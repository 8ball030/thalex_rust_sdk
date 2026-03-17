use crate::{
    models::{
        LoginParams, LoginResponse, LoginRpcResult, SetCancelOnDisconnectParams,
        SetCancelOnDisconnectResponse, SetCancelOnDisconnectRpcResult,
    },
    types::ClientError,
    ws_client::WsClient,
};

pub struct SessionManagementRpc<'a> {
    pub client: &'a WsClient,
}
impl<'a> SessionManagementRpc<'a> {
    /// Login
    /// returns: LoginRpcResult
    pub async fn login(&self, params: LoginParams) -> Result<LoginRpcResult, ClientError> {
        let result: Result<LoginResponse, ClientError> = self
            .client
            .send_rpc(
                "public/login",
                serde_json::to_value(params).expect("Failed to serialize params"),
            )
            .await;
        match result {
            Ok(res) => match res {
                LoginResponse::LoginResult(res) => Ok(res.result),
                LoginResponse::RpcErrorResponse(err) => Err(ClientError::Rpc(err)),
            },
            Err(err) => Err(err),
        }
    }

    /// Set cancel on disconnect
    /// returns: SetCancelOnDisconnectRpcResult
    pub async fn set_cancel_on_disconnect(
        &self,
        params: SetCancelOnDisconnectParams,
    ) -> Result<SetCancelOnDisconnectRpcResult, ClientError> {
        let result: Result<SetCancelOnDisconnectResponse, ClientError> = self
            .client
            .send_rpc(
                "private/set_cancel_on_disconnect",
                serde_json::to_value(params).expect("Failed to serialize params"),
            )
            .await;
        match result {
            Ok(res) => match res {
                SetCancelOnDisconnectResponse::SetCancelOnDisconnectResult(res) => Ok(res.result),
                SetCancelOnDisconnectResponse::RpcErrorResponse(err) => Err(ClientError::Rpc(err)),
            },
            Err(err) => Err(err),
        }
    }
}
