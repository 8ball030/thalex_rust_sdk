from string import Template
func_template = Template("""
    pub async fn $channel<F, Fut>(&self, $func_args mut callback: F) -> Result<String, Error>
    where
        F: FnMut($notification_model) -> Fut+ Send + 'static,
        Fut: Future<Output = ()> + Send + 'static,

    {
        let channel = format!("$channel_with_args");
        self.client.subscribe_channel(
            RequestScope::$scope,
            channel.clone(),
            move |msg: $response_model| {
                let fut = callback(msg.notification);
                tokio::spawn(fut);

            }
        ).await?;
        Ok(channel)
    }
""")

file_template = Template("""
use log::{info, warn};
use tokio::sync::mpsc;

use crate::{models::{
    $models
}, ws_client::{
    WsClient,
}, types::{
    Error, 
    RequestScope, 
}};

pub struct $tag<'a> {
    pub client: &'a WsClient,
}
impl <'a> $tag<'a> {
$functions
    
}
""")
