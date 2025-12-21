from string import Template
func_template = Template("""
    pub async fn $channel<F>(&self, $func_args mut callback: F) -> Result<(), Error>
    where
        F: FnMut($notification_model) + Send + 'static,
    {
        let channel = format!("$channel_with_args");
        // Per-subscription channel from core -> user callback
        self.client.subscribe_channel(
            RequestScope::$scope,
            channel,
            move |msg: $response_model| {
                callback(msg.notification);
            }
        ).await?;
        Ok(())
    }
""")

file_template = Template("""
use log::{info, warn};
use tokio::sync::mpsc;

use crate::{models::{
    $models
}, ws_client::{
    WsClient,
    RequestScope
}};
type Error = Box<dyn std::error::Error + Send + Sync>;

pub struct $tag<'a> {
    pub client: &'a WsClient,
}
impl <'a> $tag<'a> {
$functions
    
}
""")
