
use crate::{
    models::{
        Session, SessionNotification,
    },
    ws_client::{RequestScope, WsClient},
};
type Error = Box<dyn std::error::Error + Send + Sync>;

pub struct MmProtSubscriptions<'a> {
    pub client: &'a WsClient,
}
impl<'a> MmProtSubscriptions<'a> {
    pub async fn session_mm_protection<F>(&self, mut callback: F) -> Result<(), Error>
    where
        F: FnMut(Session) + Send + 'static,
    {
        let channel = "session.mm_protection".to_string();
        // Per-subscription channel from core -> user callback
        self.client
            .subscribe_channel(
                RequestScope::Private,
                channel,
                move |msg: SessionNotification| {
                    callback(msg.notification);
                },
            )
            .await?;
        Ok(())
    }
}
