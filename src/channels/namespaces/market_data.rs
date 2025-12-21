
use crate::{
    models::{
        BasePrice, BasePriceNotification, Book, BookNotification,
        Delay, Index, IndexComponents, IndexComponentsNotification, InstrumentsPayload,
        InstrumentsPayloadNotification, Lwt, LwtNotification, PriceIndexNotification, RecentTrades, RecentTradesNotification, RfqsPayload, RfqsPayloadNotification, Ticker,
        TickerNotification, UnderlyingStatistics, UnderlyingStatisticsNotification,
    },
    ws_client::{RequestScope, WsClient},
};
type Error = Box<dyn std::error::Error + Send + Sync>;

pub struct MarketDataSubscriptions<'a> {
    pub client: &'a WsClient,
}
impl<'a> MarketDataSubscriptions<'a> {
    pub async fn ticker<F>(
        &self,
        instrument: &str,
        delay: Delay,
        mut callback: F,
    ) -> Result<(), Error>
    where
        F: FnMut(Ticker) + Send + 'static,
    {
        let channel = format!("ticker.{instrument}.{delay}");
        // Per-subscription channel from core -> user callback
        self.client
            .subscribe_channel(
                RequestScope::Public,
                channel,
                move |msg: TickerNotification| {
                    callback(msg.notification);
                },
            )
            .await?;
        Ok(())
    }

    pub async fn book<F>(
        &self,
        instrument: &str,
        grouping: &str,
        nlevels: &str,
        delay: Delay,
        mut callback: F,
    ) -> Result<(), Error>
    where
        F: FnMut(Book) + Send + 'static,
    {
        let channel = format!("book.{instrument}.{grouping}.{nlevels}.{delay}");
        // Per-subscription channel from core -> user callback
        self.client
            .subscribe_channel(
                RequestScope::Public,
                channel,
                move |msg: BookNotification| {
                    callback(msg.notification);
                },
            )
            .await?;
        Ok(())
    }

    pub async fn lwt<F>(&self, instrument: &str, delay: Delay, mut callback: F) -> Result<(), Error>
    where
        F: FnMut(Lwt) + Send + 'static,
    {
        let channel = format!("lwt.{instrument}.{delay}");
        // Per-subscription channel from core -> user callback
        self.client
            .subscribe_channel(
                RequestScope::Public,
                channel,
                move |msg: LwtNotification| {
                    callback(msg.notification);
                },
            )
            .await?;
        Ok(())
    }

    pub async fn recent_trades<F>(
        &self,
        target: &str,
        category: &str,
        mut callback: F,
    ) -> Result<(), Error>
    where
        F: FnMut(RecentTrades) + Send + 'static,
    {
        let channel = format!("recent_trades.{target}.{category}");
        // Per-subscription channel from core -> user callback
        self.client
            .subscribe_channel(
                RequestScope::Public,
                channel,
                move |msg: RecentTradesNotification| {
                    callback(msg.notification);
                },
            )
            .await?;
        Ok(())
    }

    pub async fn price_index<F>(&self, underlying: &str, mut callback: F) -> Result<(), Error>
    where
        F: FnMut(Index) + Send + 'static,
    {
        let channel = format!("price_index.{underlying}");
        // Per-subscription channel from core -> user callback
        self.client
            .subscribe_channel(
                RequestScope::Public,
                channel,
                move |msg: PriceIndexNotification| {
                    callback(msg.notification);
                },
            )
            .await?;
        Ok(())
    }

    pub async fn underlying_statistics<F>(
        &self,
        underlying: &str,
        mut callback: F,
    ) -> Result<(), Error>
    where
        F: FnMut(UnderlyingStatistics) + Send + 'static,
    {
        let channel = format!("underlying_statistics.{underlying}");
        // Per-subscription channel from core -> user callback
        self.client
            .subscribe_channel(
                RequestScope::Public,
                channel,
                move |msg: UnderlyingStatisticsNotification| {
                    callback(msg.notification);
                },
            )
            .await?;
        Ok(())
    }

    pub async fn base_price<F>(
        &self,
        underlying: &str,
        expiration: &str,
        mut callback: F,
    ) -> Result<(), Error>
    where
        F: FnMut(BasePrice) + Send + 'static,
    {
        let channel = format!("base_price.{underlying}.{expiration}");
        // Per-subscription channel from core -> user callback
        self.client
            .subscribe_channel(
                RequestScope::Public,
                channel,
                move |msg: BasePriceNotification| {
                    callback(msg.notification);
                },
            )
            .await?;
        Ok(())
    }

    pub async fn instruments<F>(&self, mut callback: F) -> Result<(), Error>
    where
        F: FnMut(InstrumentsPayload) + Send + 'static,
    {
        let channel = "instruments.".to_string();
        // Per-subscription channel from core -> user callback
        self.client
            .subscribe_channel(
                RequestScope::Public,
                channel,
                move |msg: InstrumentsPayloadNotification| {
                    callback(msg.notification);
                },
            )
            .await?;
        Ok(())
    }

    pub async fn rfqs<F>(&self, mut callback: F) -> Result<(), Error>
    where
        F: FnMut(RfqsPayload) + Send + 'static,
    {
        let channel = "rfqs.".to_string();
        // Per-subscription channel from core -> user callback
        self.client
            .subscribe_channel(
                RequestScope::Public,
                channel,
                move |msg: RfqsPayloadNotification| {
                    callback(msg.notification);
                },
            )
            .await?;
        Ok(())
    }

    pub async fn index_components<F>(&self, underlying: &str, mut callback: F) -> Result<(), Error>
    where
        F: FnMut(IndexComponents) + Send + 'static,
    {
        let channel = format!("index_components.{underlying}");
        // Per-subscription channel from core -> user callback
        self.client
            .subscribe_channel(
                RequestScope::Public,
                channel,
                move |msg: IndexComponentsNotification| {
                    callback(msg.notification);
                },
            )
            .await?;
        Ok(())
    }
}
