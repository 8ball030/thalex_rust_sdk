
use crate::{
    models::{
        Account, AccountNotification, Session, SessionNotification,
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
        F: FnMut(Account) + Send + 'static,
    {
        let channel = "account.orders".to_string();
        // Per-subscription channel from core -> user callback
        self.client
            .subscribe_channel(
                RequestScope::Private,
                channel,
                move |msg: AccountNotification| {
                    callback(msg.notification);
                },
            )
            .await?;
        Ok(())
    }

    pub async fn account_persistent_orders<F>(&self, mut callback: F) -> Result<(), Error>
    where
        F: FnMut(Account) + Send + 'static,
    {
        let channel = "account.persistent_orders".to_string();
        // Per-subscription channel from core -> user callback
        self.client
            .subscribe_channel(
                RequestScope::Private,
                channel,
                move |msg: AccountNotification| {
                    callback(msg.notification);
                },
            )
            .await?;
        Ok(())
    }

    pub async fn session_orders<F>(&self, mut callback: F) -> Result<(), Error>
    where
        F: FnMut(Session) + Send + 'static,
    {
        let channel = "session.orders".to_string();
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

    pub async fn account_trade_history<F>(&self, mut callback: F) -> Result<(), Error>
    where
        F: FnMut(Account) + Send + 'static,
    {
        let channel = "account.trade_history".to_string();
        // Per-subscription channel from core -> user callback
        self.client
            .subscribe_channel(
                RequestScope::Private,
                channel,
                move |msg: AccountNotification| {
                    callback(msg.notification);
                },
            )
            .await?;
        Ok(())
    }

    pub async fn account_order_history<F>(&self, mut callback: F) -> Result<(), Error>
    where
        F: FnMut(Account) + Send + 'static,
    {
        let channel = "account.order_history".to_string();
        // Per-subscription channel from core -> user callback
        self.client
            .subscribe_channel(
                RequestScope::Private,
                channel,
                move |msg: AccountNotification| {
                    callback(msg.notification);
                },
            )
            .await?;
        Ok(())
    }

    pub async fn account_portfolio<F>(&self, mut callback: F) -> Result<(), Error>
    where
        F: FnMut(Account) + Send + 'static,
    {
        let channel = "account.portfolio".to_string();
        // Per-subscription channel from core -> user callback
        self.client
            .subscribe_channel(
                RequestScope::Private,
                channel,
                move |msg: AccountNotification| {
                    callback(msg.notification);
                },
            )
            .await?;
        Ok(())
    }

    pub async fn account_summary<F>(&self, mut callback: F) -> Result<(), Error>
    where
        F: FnMut(Account) + Send + 'static,
    {
        let channel = "account.summary".to_string();
        // Per-subscription channel from core -> user callback
        self.client
            .subscribe_channel(
                RequestScope::Private,
                channel,
                move |msg: AccountNotification| {
                    callback(msg.notification);
                },
            )
            .await?;
        Ok(())
    }

    pub async fn account_rfqs<F>(&self, mut callback: F) -> Result<(), Error>
    where
        F: FnMut(Account) + Send + 'static,
    {
        let channel = "account.rfqs".to_string();
        // Per-subscription channel from core -> user callback
        self.client
            .subscribe_channel(
                RequestScope::Private,
                channel,
                move |msg: AccountNotification| {
                    callback(msg.notification);
                },
            )
            .await?;
        Ok(())
    }

    pub async fn account_rfq_history<F>(&self, mut callback: F) -> Result<(), Error>
    where
        F: FnMut(Account) + Send + 'static,
    {
        let channel = "account.rfq_history".to_string();
        // Per-subscription channel from core -> user callback
        self.client
            .subscribe_channel(
                RequestScope::Private,
                channel,
                move |msg: AccountNotification| {
                    callback(msg.notification);
                },
            )
            .await?;
        Ok(())
    }
}
