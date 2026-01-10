use crate::{
    models::{
        AccountOrderHistoryNotification, AccountOrderHistoryPayload, AccountOrdersNotification,
        AccountOrdersPayload, AccountPersistentOrdersNotification, AccountPersistentOrdersPayload,
        AccountPortfolioNotification, AccountPortfolioPayload, AccountRfqHistoryNotification,
        AccountRfqHistoryPayload, AccountRfqsNotification, AccountRfqsPayload, AccountSummary,
        AccountSummaryNotification, AccountTradeHistoryNotification, AccountTradeHistoryPayload,
        SessionOrdersNotification, SessionOrdersPayload,
    },
    types::{Error, RequestScope},
    ws_client::WsClient,
};

pub struct AccountingSubscriptions<'a> {
    pub client: &'a WsClient,
}
impl<'a> AccountingSubscriptions<'a> {
    pub async fn account_orders<F, Fut>(&self, mut callback: F) -> Result<String, Error>
    where
        F: FnMut(AccountOrdersPayload) -> Fut + Send + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let channel = "account.orders".to_string();
        self.client
            .subscribe_channel(
                RequestScope::Private,
                channel.clone(),
                move |msg: AccountOrdersNotification| {
                    let fut = callback(msg.notification);
                    tokio::spawn(fut);
                },
            )
            .await?;
        Ok(channel)
    }

    pub async fn account_persistent_orders<F, Fut>(&self, mut callback: F) -> Result<String, Error>
    where
        F: FnMut(AccountPersistentOrdersPayload) -> Fut + Send + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let channel = "account.persistent_orders".to_string();
        self.client
            .subscribe_channel(
                RequestScope::Private,
                channel.clone(),
                move |msg: AccountPersistentOrdersNotification| {
                    let fut = callback(msg.notification);
                    tokio::spawn(fut);
                },
            )
            .await?;
        Ok(channel)
    }

    pub async fn session_orders<F, Fut>(&self, mut callback: F) -> Result<String, Error>
    where
        F: FnMut(SessionOrdersPayload) -> Fut + Send + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let channel = "session.orders".to_string();
        self.client
            .subscribe_channel(
                RequestScope::Private,
                channel.clone(),
                move |msg: SessionOrdersNotification| {
                    let fut = callback(msg.notification);
                    tokio::spawn(fut);
                },
            )
            .await?;
        Ok(channel)
    }

    pub async fn account_trade_history<F, Fut>(&self, mut callback: F) -> Result<String, Error>
    where
        F: FnMut(AccountTradeHistoryPayload) -> Fut + Send + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let channel = "account.trade_history".to_string();
        self.client
            .subscribe_channel(
                RequestScope::Private,
                channel.clone(),
                move |msg: AccountTradeHistoryNotification| {
                    let fut = callback(msg.notification);
                    tokio::spawn(fut);
                },
            )
            .await?;
        Ok(channel)
    }

    pub async fn account_order_history<F, Fut>(&self, mut callback: F) -> Result<String, Error>
    where
        F: FnMut(AccountOrderHistoryPayload) -> Fut + Send + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let channel = "account.order_history".to_string();
        self.client
            .subscribe_channel(
                RequestScope::Private,
                channel.clone(),
                move |msg: AccountOrderHistoryNotification| {
                    let fut = callback(msg.notification);
                    tokio::spawn(fut);
                },
            )
            .await?;
        Ok(channel)
    }

    pub async fn account_portfolio<F, Fut>(&self, mut callback: F) -> Result<String, Error>
    where
        F: FnMut(AccountPortfolioPayload) -> Fut + Send + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let channel = "account.portfolio".to_string();
        self.client
            .subscribe_channel(
                RequestScope::Private,
                channel.clone(),
                move |msg: AccountPortfolioNotification| {
                    let fut = callback(msg.notification);
                    tokio::spawn(fut);
                },
            )
            .await?;
        Ok(channel)
    }

    pub async fn account_summary<F, Fut>(&self, mut callback: F) -> Result<String, Error>
    where
        F: FnMut(AccountSummary) -> Fut + Send + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let channel = "account.summary".to_string();
        self.client
            .subscribe_channel(
                RequestScope::Private,
                channel.clone(),
                move |msg: AccountSummaryNotification| {
                    let fut = callback(msg.notification);
                    tokio::spawn(fut);
                },
            )
            .await?;
        Ok(channel)
    }

    pub async fn account_rfqs<F, Fut>(&self, mut callback: F) -> Result<String, Error>
    where
        F: FnMut(AccountRfqsPayload) -> Fut + Send + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let channel = "account.rfqs".to_string();
        self.client
            .subscribe_channel(
                RequestScope::Private,
                channel.clone(),
                move |msg: AccountRfqsNotification| {
                    let fut = callback(msg.notification);
                    tokio::spawn(fut);
                },
            )
            .await?;
        Ok(channel)
    }

    pub async fn account_rfq_history<F, Fut>(&self, mut callback: F) -> Result<String, Error>
    where
        F: FnMut(AccountRfqHistoryPayload) -> Fut + Send + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let channel = "account.rfq_history".to_string();
        self.client
            .subscribe_channel(
                RequestScope::Private,
                channel.clone(),
                move |msg: AccountRfqHistoryNotification| {
                    let fut = callback(msg.notification);
                    tokio::spawn(fut);
                },
            )
            .await?;
        Ok(channel)
    }
}
