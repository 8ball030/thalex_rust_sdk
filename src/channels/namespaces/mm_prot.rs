use crate::{
    models::{SessionMmProtectionNotification, SessionMmProtectionPayload},
    types::{Error, RequestScope},
    ws_client::WsClient,
};

pub struct MmProtSubscriptions<'a> {
    pub client: &'a WsClient,
}
impl<'a> MmProtSubscriptions<'a> {
    pub async fn session_mm_protection<F, Fut>(&self, mut callback: F) -> Result<String, Error>
    where
        F: FnMut(SessionMmProtectionPayload) -> Fut + Send + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let channel = "session.mm_protection".to_string();
        self.client
            .subscribe_channel(
                RequestScope::Private,
                channel.clone(),
                move |msg: SessionMmProtectionNotification| {
                    let fut = callback(msg.notification);
                    tokio::spawn(fut);
                },
            )
            .await?;
        Ok(channel)
    }
}
