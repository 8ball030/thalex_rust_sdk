
use crate::{
    models::{
        Account, AccountNotification,
    },
    ws_client::{RequestScope, WsClient},
};
type Error = Box<dyn std::error::Error + Send + Sync>;

pub struct ConditionalSubscriptions<'a> {
    pub client: &'a WsClient,
}
impl<'a> ConditionalSubscriptions<'a> {
    pub async fn account_conditional_orders<F>(&self, mut callback: F) -> Result<(), Error>
    where
        F: FnMut(Account) + Send + 'static,
    {
        let channel = "account.conditional_orders".to_string();
        // Per-subscription channel from core -> user callback
        self.client
            .subscribe_channel(
                RequestScope::Private,
                channel,
                move |msg: AccountNotification| {
                    callback(msg.notification);
                },
            )
            .await?;
        Ok(())
    }
}
