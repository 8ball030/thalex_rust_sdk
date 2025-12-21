
use crate::{
    models::{
        MmRfqQuotesNotification,
        MmRfqQuotesPayload, MmRfqsNotification, MmRfqsPayload,
    },
    ws_client::{RequestScope, WsClient},
};
type Error = Box<dyn std::error::Error + Send + Sync>;

pub struct MmRfqSubscriptions<'a> {
    pub client: &'a WsClient,
}
impl<'a> MmRfqSubscriptions<'a> {
    pub async fn mm_rfqs<F, Fut>(&self, mut callback: F) -> Result<(), Error>
    where
        F: FnMut(MmRfqsPayload) -> Fut + Send + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let channel = "mm.rfqs".to_string();
        self.client
            .subscribe_channel(
                RequestScope::Private,
                channel,
                move |msg: MmRfqsNotification| {
                    let fut = callback(msg.notification);
                    tokio::spawn(fut);
                },
            )
            .await?;
        Ok(())
    }

    pub async fn mm_rfq_quotes<F, Fut>(&self, mut callback: F) -> Result<(), Error>
    where
        F: FnMut(MmRfqQuotesPayload) -> Fut + Send + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let channel = "mm.rfq_quotes".to_string();
        self.client
            .subscribe_channel(
                RequestScope::Private,
                channel,
                move |msg: MmRfqQuotesNotification| {
                    let fut = callback(msg.notification);
                    tokio::spawn(fut);
                },
            )
            .await?;
        Ok(())
    }
}
