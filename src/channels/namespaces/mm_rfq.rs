
use crate::{
    models::{
        MmRfqQuotesNotification, MmRfqQuotesPayload, MmRfqsNotification, MmRfqsPayload,
    },
    ws_client::{RequestScope, WsClient},
};
type Error = Box<dyn std::error::Error + Send + Sync>;

pub struct MmRfqSubscriptions<'a> {
    pub client: &'a WsClient,
}
impl<'a> MmRfqSubscriptions<'a> {
    pub async fn mm_rfqs<F>(&self, mut callback: F) -> Result<(), Error>
    where
        F: FnMut(MmRfqsPayload) + Send + 'static,
    {
        let channel = "mm.rfqs".to_string();
        // Per-subscription channel from core -> user callback
        self.client
            .subscribe_channel(
                RequestScope::Private,
                channel,
                move |msg: MmRfqsNotification| {
                    callback(msg.notification);
                },
            )
            .await?;
        Ok(())
    }

    pub async fn mm_rfq_quotes<F>(&self, mut callback: F) -> Result<(), Error>
    where
        F: FnMut(MmRfqQuotesPayload) + Send + 'static,
    {
        let channel = "mm.rfq_quotes".to_string();
        // Per-subscription channel from core -> user callback
        self.client
            .subscribe_channel(
                RequestScope::Private,
                channel,
                move |msg: MmRfqQuotesNotification| {
                    callback(msg.notification);
                },
            )
            .await?;
        Ok(())
    }
}
