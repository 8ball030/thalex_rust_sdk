
use crate::{
    models::{
        BannersNotification, BannersPayload, SystemEvent, SystemNotification,
    },
    ws_client::{RequestScope, WsClient},
};
type Error = Box<dyn std::error::Error + Send + Sync>;

pub struct SystemSubscriptions<'a> {
    pub client: &'a WsClient,
}
impl<'a> SystemSubscriptions<'a> {
    pub async fn system<F, Fut>(&self, mut callback: F) -> Result<(), Error>
    where
        F: FnMut(SystemEvent) -> Fut + Send + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let channel = "system".to_string();
        self.client
            .subscribe_channel(
                RequestScope::Public,
                channel,
                move |msg: SystemNotification| {
                    let fut = callback(msg.notification);
                    tokio::spawn(fut);
                },
            )
            .await?;
        Ok(())
    }

    pub async fn banners<F, Fut>(&self, mut callback: F) -> Result<(), Error>
    where
        F: FnMut(BannersPayload) -> Fut + Send + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let channel = "banners".to_string();
        self.client
            .subscribe_channel(
                RequestScope::Public,
                channel,
                move |msg: BannersNotification| {
                    let fut = callback(msg.notification);
                    tokio::spawn(fut);
                },
            )
            .await?;
        Ok(())
    }
}
