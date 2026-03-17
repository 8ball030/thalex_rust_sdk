from string import Template
method_template = Template("""
    /// $description
    /// returns: $return_model
    pub async fn $method_name(&self, params: $params) -> Result<$return_model, ClientError> {
        let result: Result<$response_model, ClientError> = self
            .client
            .send_rpc(
                "$method",
                serde_json::to_value(params).expect("Failed to serialize params"),
            )
            .await;
        match result {
            Ok(res) => {
                match res {
                    $response_model::$result_model(res) => Ok(res.result),
                    $response_model::RpcErrorResponse(err) => Err(ClientError::Rpc(err)),
                }
            }
            Err(err) => Err(err),
        }
    }
""")
no_param_method_template = Template("""
    /// $description
    /// returns: $return_model
    pub async fn $method_name(&self, ) -> Result<$return_model, ClientError> {
        let result: Result<$response_model, ClientError> = self
            .client
            .send_rpc(
                "$method",
                serde_json::to_value({}).expect("Failed to serialize params"),
            )
            .await;
        match result {
            Ok(res) => {
                match res {
                    $response_model::$result_model(res) => Ok(res.result),
                    $response_model::RpcErrorResponse(err) => Err(ClientError::Rpc(err)),
                }
            }
            Err(err) => Err(err),
        }
    }
""")
file_template = Template("""

use crate::{models::{
    $models
}, ws_client::{
    WsClient,
}, types::{
    Error, ClientError,
}};
use serde_json::Value;
use rust_decimal::Decimal;

pub struct $tag<'a> {
    pub client: &'a WsClient,
}
impl <'a> $tag<'a> {
$functions
    
}
""")
