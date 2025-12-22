use crate::{
    models::{
        BasePrice, BasePriceNotification, Book, BookNotification, Delay, Index, IndexComponents,
        IndexComponentsNotification, InstrumentsNotification, InstrumentsPayload, Lwt,
        LwtNotification, PriceIndexNotification, RecentTrades, RecentTradesNotification,
        RfqsNotification, RfqsPayload, Ticker, TickerNotification, UnderlyingStatistics,
        UnderlyingStatisticsNotification,
    },
    ws_client::{RequestScope, WsClient},
};
type Error = Box<dyn std::error::Error + Send + Sync>;

pub struct MarketDataSubscriptions<'a> {
    pub client: &'a WsClient,
}
impl<'a> MarketDataSubscriptions<'a> {
    pub async fn ticker<F, Fut>(
        &self,
        instrument: &str,
        delay: Delay,
        mut callback: F,
    ) -> Result<(), Error>
    where
        F: FnMut(Ticker) -> Fut + Send + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let channel = format!("ticker.{instrument}.{delay}");
        self.client
            .subscribe_channel(
                RequestScope::Public,
                channel,
                move |msg: TickerNotification| {
                    let fut = callback(msg.notification);
                    tokio::spawn(fut);
                },
            )
            .await?;
        Ok(())
    }

    pub async fn book<F, Fut>(
        &self,
        instrument: &str,
        grouping: &str,
        nlevels: &str,
        delay: Delay,
        mut callback: F,
    ) -> Result<(), Error>
    where
        F: FnMut(Book) -> Fut + Send + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let channel = format!("book.{instrument}.{grouping}.{nlevels}.{delay}");
        self.client
            .subscribe_channel(
                RequestScope::Public,
                channel,
                move |msg: BookNotification| {
                    let fut = callback(msg.notification);
                    tokio::spawn(fut);
                },
            )
            .await?;
        Ok(())
    }

    pub async fn lwt<F, Fut>(
        &self,
        instrument: &str,
        delay: Delay,
        mut callback: F,
    ) -> Result<(), Error>
    where
        F: FnMut(Lwt) -> Fut + Send + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let channel = format!("lwt.{instrument}.{delay}");
        self.client
            .subscribe_channel(
                RequestScope::Public,
                channel,
                move |msg: LwtNotification| {
                    let fut = callback(msg.notification);
                    tokio::spawn(fut);
                },
            )
            .await?;
        Ok(())
    }

    pub async fn recent_trades<F, Fut>(
        &self,
        target: &str,
        category: &str,
        mut callback: F,
    ) -> Result<(), Error>
    where
        F: FnMut(RecentTrades) -> Fut + Send + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let channel = format!("recent_trades.{target}.{category}");
        self.client
            .subscribe_channel(
                RequestScope::Public,
                channel,
                move |msg: RecentTradesNotification| {
                    let fut = callback(msg.notification);
                    tokio::spawn(fut);
                },
            )
            .await?;
        Ok(())
    }

    pub async fn price_index<F, Fut>(&self, underlying: &str, mut callback: F) -> Result<(), Error>
    where
        F: FnMut(Index) -> Fut + Send + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let channel = format!("price_index.{underlying}");
        self.client
            .subscribe_channel(
                RequestScope::Public,
                channel,
                move |msg: PriceIndexNotification| {
                    let fut = callback(msg.notification);
                    tokio::spawn(fut);
                },
            )
            .await?;
        Ok(())
    }

    pub async fn underlying_statistics<F, Fut>(
        &self,
        underlying: &str,
        mut callback: F,
    ) -> Result<(), Error>
    where
        F: FnMut(UnderlyingStatistics) -> Fut + Send + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let channel = format!("underlying_statistics.{underlying}");
        self.client
            .subscribe_channel(
                RequestScope::Public,
                channel,
                move |msg: UnderlyingStatisticsNotification| {
                    let fut = callback(msg.notification);
                    tokio::spawn(fut);
                },
            )
            .await?;
        Ok(())
    }

    pub async fn base_price<F, Fut>(
        &self,
        underlying: &str,
        expiration: &str,
        mut callback: F,
    ) -> Result<(), Error>
    where
        F: FnMut(BasePrice) -> Fut + Send + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let channel = format!("base_price.{underlying}.{expiration}");
        self.client
            .subscribe_channel(
                RequestScope::Public,
                channel,
                move |msg: BasePriceNotification| {
                    let fut = callback(msg.notification);
                    tokio::spawn(fut);
                },
            )
            .await?;
        Ok(())
    }

    pub async fn instruments<F, Fut>(&self, mut callback: F) -> Result<(), Error>
    where
        F: FnMut(InstrumentsPayload) -> Fut + Send + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let channel = "instruments".to_string();
        self.client
            .subscribe_channel(
                RequestScope::Public,
                channel,
                move |msg: InstrumentsNotification| {
                    let fut = callback(msg.notification);
                    tokio::spawn(fut);
                },
            )
            .await?;
        Ok(())
    }

    pub async fn rfqs<F, Fut>(&self, mut callback: F) -> Result<(), Error>
    where
        F: FnMut(RfqsPayload) -> Fut + Send + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let channel = "rfqs".to_string();
        self.client
            .subscribe_channel(
                RequestScope::Public,
                channel,
                move |msg: RfqsNotification| {
                    let fut = callback(msg.notification);
                    tokio::spawn(fut);
                },
            )
            .await?;
        Ok(())
    }

    pub async fn index_components<F, Fut>(
        &self,
        underlying: &str,
        mut callback: F,
    ) -> Result<(), Error>
    where
        F: FnMut(IndexComponents) -> Fut + Send + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let channel = format!("index_components.{underlying}");
        self.client
            .subscribe_channel(
                RequestScope::Public,
                channel,
                move |msg: IndexComponentsNotification| {
                    let fut = callback(msg.notification);
                    tokio::spawn(fut);
                },
            )
            .await?;
        Ok(())
    }
}
