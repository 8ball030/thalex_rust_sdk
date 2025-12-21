
use crate::{
    models::{
        AccountOrderHistoryNotification, AccountOrderHistoryPayload,
        AccountOrdersNotification, AccountOrdersPayload, AccountPersistentOrdersNotification,
        AccountPersistentOrdersPayload, AccountPortfolioNotification, AccountPortfolioPayload,
        AccountRfqHistoryNotification, AccountRfqHistoryPayload, AccountRfqsNotification,
        AccountRfqsPayload, AccountSummary, AccountSummaryNotification,
        AccountTradeHistoryNotification, AccountTradeHistoryPayload,
        SessionOrdersNotification, SessionOrdersPayload,
    },
    ws_client::{RequestScope, WsClient},
};
type Error = Box<dyn std::error::Error + Send + Sync>;

pub struct AccountingSubscriptions<'a> {
    pub client: &'a WsClient,
}
impl<'a> AccountingSubscriptions<'a> {
    pub async fn account_orders<F>(&self, mut callback: F) -> Result<(), Error>
    where
        F: FnMut(AccountOrdersPayload) + Send + 'static,
    {
        let channel = "account.orders".to_string();
        // Per-subscription channel from core -> user callback
        self.client
            .subscribe_channel(
                RequestScope::Private,
                channel,
                move |msg: AccountOrdersNotification| {
                    callback(msg.notification);
                },
            )
            .await?;
        Ok(())
    }

    pub async fn account_persistent_orders<F>(&self, mut callback: F) -> Result<(), Error>
    where
        F: FnMut(AccountPersistentOrdersPayload) + Send + 'static,
    {
        let channel = "account.persistent_orders".to_string();
        // Per-subscription channel from core -> user callback
        self.client
            .subscribe_channel(
                RequestScope::Private,
                channel,
                move |msg: AccountPersistentOrdersNotification| {
                    callback(msg.notification);
                },
            )
            .await?;
        Ok(())
    }

    pub async fn session_orders<F>(&self, mut callback: F) -> Result<(), Error>
    where
        F: FnMut(SessionOrdersPayload) + Send + 'static,
    {
        let channel = "session.orders".to_string();
        // Per-subscription channel from core -> user callback
        self.client
            .subscribe_channel(
                RequestScope::Private,
                channel,
                move |msg: SessionOrdersNotification| {
                    callback(msg.notification);
                },
            )
            .await?;
        Ok(())
    }

    pub async fn account_trade_history<F>(&self, mut callback: F) -> Result<(), Error>
    where
        F: FnMut(AccountTradeHistoryPayload) + Send + 'static,
    {
        let channel = "account.trade_history".to_string();
        // Per-subscription channel from core -> user callback
        self.client
            .subscribe_channel(
                RequestScope::Private,
                channel,
                move |msg: AccountTradeHistoryNotification| {
                    callback(msg.notification);
                },
            )
            .await?;
        Ok(())
    }

    pub async fn account_order_history<F>(&self, mut callback: F) -> Result<(), Error>
    where
        F: FnMut(AccountOrderHistoryPayload) + Send + 'static,
    {
        let channel = "account.order_history".to_string();
        // Per-subscription channel from core -> user callback
        self.client
            .subscribe_channel(
                RequestScope::Private,
                channel,
                move |msg: AccountOrderHistoryNotification| {
                    callback(msg.notification);
                },
            )
            .await?;
        Ok(())
    }

    pub async fn account_portfolio<F>(&self, mut callback: F) -> Result<(), Error>
    where
        F: FnMut(AccountPortfolioPayload) + Send + 'static,
    {
        let channel = "account.portfolio".to_string();
        // Per-subscription channel from core -> user callback
        self.client
            .subscribe_channel(
                RequestScope::Private,
                channel,
                move |msg: AccountPortfolioNotification| {
                    callback(msg.notification);
                },
            )
            .await?;
        Ok(())
    }

    pub async fn account_summary<F>(&self, mut callback: F) -> Result<(), Error>
    where
        F: FnMut(AccountSummary) + Send + 'static,
    {
        let channel = "account.summary".to_string();
        // Per-subscription channel from core -> user callback
        self.client
            .subscribe_channel(
                RequestScope::Private,
                channel,
                move |msg: AccountSummaryNotification| {
                    callback(msg.notification);
                },
            )
            .await?;
        Ok(())
    }

    pub async fn account_rfqs<F>(&self, mut callback: F) -> Result<(), Error>
    where
        F: FnMut(AccountRfqsPayload) + Send + 'static,
    {
        let channel = "account.rfqs".to_string();
        // Per-subscription channel from core -> user callback
        self.client
            .subscribe_channel(
                RequestScope::Private,
                channel,
                move |msg: AccountRfqsNotification| {
                    callback(msg.notification);
                },
            )
            .await?;
        Ok(())
    }

    pub async fn account_rfq_history<F>(&self, mut callback: F) -> Result<(), Error>
    where
        F: FnMut(AccountRfqHistoryPayload) + Send + 'static,
    {
        let channel = "account.rfq_history".to_string();
        // Per-subscription channel from core -> user callback
        self.client
            .subscribe_channel(
                RequestScope::Private,
                channel,
                move |msg: AccountRfqHistoryNotification| {
                    callback(msg.notification);
                },
            )
            .await?;
        Ok(())
    }
}
