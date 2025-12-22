use crate::{
    models::{AccountBotsNotification, AccountBotsPayload},
    ws_client::{RequestScope, WsClient},
};
type Error = Box<dyn std::error::Error + Send + Sync>;

pub struct BotSubscriptions<'a> {
    pub client: &'a WsClient,
}
impl<'a> BotSubscriptions<'a> {
    pub async fn account_bots<F, Fut>(&self, mut callback: F) -> Result<(), Error>
    where
        F: FnMut(AccountBotsPayload) -> Fut + Send + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let channel = "account.bots".to_string();
        self.client
            .subscribe_channel(
                RequestScope::Private,
                channel,
                move |msg: AccountBotsNotification| {
                    let fut = callback(msg.notification);
                    tokio::spawn(fut);
                },
            )
            .await?;
        Ok(())
    }
}
