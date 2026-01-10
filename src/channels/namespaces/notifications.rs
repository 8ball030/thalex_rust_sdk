use crate::{
    models::{Notifications, UserInboxNotificationsNotification},
    types::{Error, RequestScope},
    ws_client::WsClient,
};

pub struct NotificationsSubscriptions<'a> {
    pub client: &'a WsClient,
}
impl<'a> NotificationsSubscriptions<'a> {
    pub async fn user_inbox_notifications<F, Fut>(&self, mut callback: F) -> Result<String, Error>
    where
        F: FnMut(Notifications) -> Fut + Send + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let channel = "user.inbox_notifications".to_string();
        self.client
            .subscribe_channel(
                RequestScope::Private,
                channel.clone(),
                move |msg: UserInboxNotificationsNotification| {
                    let fut = callback(msg.notification);
                    tokio::spawn(fut);
                },
            )
            .await?;
        Ok(channel)
    }
}
