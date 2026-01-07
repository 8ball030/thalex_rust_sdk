use crate::{
    models::{AccountConditionalOrdersNotification, AccountConditionalOrdersPayload},
    types::{Error, RequestScope},
    ws_client::WsClient,
};

pub struct ConditionalSubscriptions<'a> {
    pub client: &'a WsClient,
}
impl<'a> ConditionalSubscriptions<'a> {
    pub async fn account_conditional_orders<F, Fut>(&self, mut callback: F) -> Result<(), Error>
    where
        F: FnMut(AccountConditionalOrdersPayload) -> Fut + Send + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let channel = "account.conditional_orders".to_string();
        self.client
            .subscribe_channel(
                RequestScope::Private,
                channel,
                move |msg: AccountConditionalOrdersNotification| {
                    let fut = callback(msg.notification);
                    tokio::spawn(fut);
                },
            )
            .await?;
        Ok(())
    }
}
