use crate::{
    models::{Notifications, UserInboxNotificationsNotification},
    ws_client::{RequestScope, WsClient},
};
type Error = Box<dyn std::error::Error + Send + Sync>;

pub struct NotificationsSubscriptions<'a> {
    pub client: &'a WsClient,
}
impl<'a> NotificationsSubscriptions<'a> {
    pub async fn user_inbox_notifications<F, Fut>(&self, mut callback: F) -> Result<(), Error>
    where
        F: FnMut(Notifications) -> Fut + Send + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let channel = "user.inbox_notifications".to_string();
        self.client
            .subscribe_channel(
                RequestScope::Private,
                channel,
                move |msg: UserInboxNotificationsNotification| {
                    let fut = callback(msg.notification);
                    tokio::spawn(fut);
                },
            )
            .await?;
        Ok(())
    }
}
