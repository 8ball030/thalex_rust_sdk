use log::{info, warn};
use tokio::sync::mpsc;

use crate::{
    models::{
        Account, AccountNotification, BasePrice, BasePriceNotification, Book, BookNotification,
        Delay, Index, IndexComponents, IndexComponentsNotification, InstrumentsPayload,
        InstrumentsPayloadNotification, Lwt, LwtNotification, Mm, MmNotification, PriceIndexNotification, RecentTrades, RecentTradesNotification, RfqsPayload, RfqsPayloadNotification, Session, SessionNotification, Ticker,
        TickerNotification, UnderlyingStatistics, UnderlyingStatisticsNotification,
    },
    ws_client::WsClient,
};

type Error = Box<dyn std::error::Error + Send + Sync>;

pub struct Subscriptions<'a> {
    pub client: &'a WsClient,
}
impl<'a> Subscriptions<'a> {
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
        let (tx, mut rx) = mpsc::unbounded_channel::<String>();

        {
            let mut subs = self.client.subscriptions.lock().await;
            subs.insert(channel.clone(), tx);
        }

        let msg = serde_json::json!({
            "method": "public/subscribe",
            "params": {
                "channels": [channel]
            }
        });

        self.client.send_json(msg)?;

        // Spawn callback task
        tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                // Parses into a json value initally
                let parsed_msg: TickerNotification = match serde_json::from_str(&msg) {
                    Ok(m) => m,
                    Err(e) => {
                        warn!("Failed to parse channel message: {e}; raw: {msg}");
                        continue;
                    }
                };
                callback(parsed_msg.notification);
            }
        });

        info!("Subscribed to channel: {channel}");
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
        let (tx, mut rx) = mpsc::unbounded_channel::<String>();

        {
            let mut subs = self.client.subscriptions.lock().await;
            subs.insert(channel.clone(), tx);
        }

        let msg = serde_json::json!({
            "method": "public/subscribe",
            "params": {
                "channels": [channel]
            }
        });

        self.client.send_json(msg)?;

        // Spawn callback task
        tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                // Parses into a json value initally
                let parsed_msg: BookNotification = match serde_json::from_str(&msg) {
                    Ok(m) => m,
                    Err(e) => {
                        warn!("Failed to parse channel message: {e}; raw: {msg}");
                        continue;
                    }
                };
                callback(parsed_msg.notification);
            }
        });

        info!("Subscribed to channel: {channel}");
        Ok(())
    }

    pub async fn lwt<F>(&self, instrument: &str, delay: Delay, mut callback: F) -> Result<(), Error>
    where
        F: FnMut(Lwt) + Send + 'static,
    {
        let channel = format!("lwt.{instrument}.{delay}");

        // Per-subscription channel from core -> user callback
        let (tx, mut rx) = mpsc::unbounded_channel::<String>();

        {
            let mut subs = self.client.subscriptions.lock().await;
            subs.insert(channel.clone(), tx);
        }

        let msg = serde_json::json!({
            "method": "public/subscribe",
            "params": {
                "channels": [channel]
            }
        });

        self.client.send_json(msg)?;

        // Spawn callback task
        tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                // Parses into a json value initally
                let parsed_msg: LwtNotification = match serde_json::from_str(&msg) {
                    Ok(m) => m,
                    Err(e) => {
                        warn!("Failed to parse channel message: {e}; raw: {msg}");
                        continue;
                    }
                };
                callback(parsed_msg.notification);
            }
        });

        info!("Subscribed to channel: {channel}");
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
        let (tx, mut rx) = mpsc::unbounded_channel::<String>();

        {
            let mut subs = self.client.subscriptions.lock().await;
            subs.insert(channel.clone(), tx);
        }

        let msg = serde_json::json!({
            "method": "public/subscribe",
            "params": {
                "channels": [channel]
            }
        });

        self.client.send_json(msg)?;

        // Spawn callback task
        tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                // Parses into a json value initally
                let parsed_msg: RecentTradesNotification = match serde_json::from_str(&msg) {
                    Ok(m) => m,
                    Err(e) => {
                        warn!("Failed to parse channel message: {e}; raw: {msg}");
                        continue;
                    }
                };
                callback(parsed_msg.notification);
            }
        });

        info!("Subscribed to channel: {channel}");
        Ok(())
    }

    pub async fn price_index<F>(&self, underlying: &str, mut callback: F) -> Result<(), Error>
    where
        F: FnMut(Index) + Send + 'static,
    {
        let channel = format!("price_index.{underlying}");

        // Per-subscription channel from core -> user callback
        let (tx, mut rx) = mpsc::unbounded_channel::<String>();

        {
            let mut subs = self.client.subscriptions.lock().await;
            subs.insert(channel.clone(), tx);
        }

        let msg = serde_json::json!({
            "method": "public/subscribe",
            "params": {
                "channels": [channel]
            }
        });

        self.client.send_json(msg)?;

        // Spawn callback task
        tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                // Parses into a json value initally
                let parsed_msg: PriceIndexNotification = match serde_json::from_str(&msg) {
                    Ok(m) => m,
                    Err(e) => {
                        warn!("Failed to parse channel message: {e}; raw: {msg}");
                        continue;
                    }
                };
                callback(parsed_msg.notification);
            }
        });

        info!("Subscribed to channel: {channel}");
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
        let (tx, mut rx) = mpsc::unbounded_channel::<String>();

        {
            let mut subs = self.client.subscriptions.lock().await;
            subs.insert(channel.clone(), tx);
        }

        let msg = serde_json::json!({
            "method": "public/subscribe",
            "params": {
                "channels": [channel]
            }
        });

        self.client.send_json(msg)?;

        // Spawn callback task
        tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                // Parses into a json value initally
                let parsed_msg: UnderlyingStatisticsNotification = match serde_json::from_str(&msg)
                {
                    Ok(m) => m,
                    Err(e) => {
                        warn!("Failed to parse channel message: {e}; raw: {msg}");
                        continue;
                    }
                };
                callback(parsed_msg.notification);
            }
        });

        info!("Subscribed to channel: {channel}");
        Ok(())
    }

    pub async fn session_mm_protection<F>(&self, mut callback: F) -> Result<(), Error>
    where
        F: FnMut(Session) + Send + 'static,
    {
        let channel = "session_mm_protection.".to_string();

        // Per-subscription channel from core -> user callback
        let (tx, mut rx) = mpsc::unbounded_channel::<String>();

        {
            let mut subs = self.client.subscriptions.lock().await;
            subs.insert(channel.clone(), tx);
        }

        let msg = serde_json::json!({
            "method": "public/subscribe",
            "params": {
                "channels": [channel]
            }
        });

        self.client.send_json(msg)?;

        // Spawn callback task
        tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                // Parses into a json value initally
                let parsed_msg: SessionNotification = match serde_json::from_str(&msg) {
                    Ok(m) => m,
                    Err(e) => {
                        warn!("Failed to parse channel message: {e}; raw: {msg}");
                        continue;
                    }
                };
                callback(parsed_msg.notification);
            }
        });

        info!("Subscribed to channel: {channel}");
        Ok(())
    }

    pub async fn mm_rfqs<F>(&self, mut callback: F) -> Result<(), Error>
    where
        F: FnMut(Mm) + Send + 'static,
    {
        let channel = "mm_rfqs.".to_string();

        // Per-subscription channel from core -> user callback
        let (tx, mut rx) = mpsc::unbounded_channel::<String>();

        {
            let mut subs = self.client.subscriptions.lock().await;
            subs.insert(channel.clone(), tx);
        }

        let msg = serde_json::json!({
            "method": "public/subscribe",
            "params": {
                "channels": [channel]
            }
        });

        self.client.send_json(msg)?;

        // Spawn callback task
        tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                // Parses into a json value initally
                let parsed_msg: MmNotification = match serde_json::from_str(&msg) {
                    Ok(m) => m,
                    Err(e) => {
                        warn!("Failed to parse channel message: {e}; raw: {msg}");
                        continue;
                    }
                };
                callback(parsed_msg.notification);
            }
        });

        info!("Subscribed to channel: {channel}");
        Ok(())
    }

    pub async fn mm_rfq_quotes<F>(&self, mut callback: F) -> Result<(), Error>
    where
        F: FnMut(Mm) + Send + 'static,
    {
        let channel = "mm_rfq_quotes.".to_string();

        // Per-subscription channel from core -> user callback
        let (tx, mut rx) = mpsc::unbounded_channel::<String>();

        {
            let mut subs = self.client.subscriptions.lock().await;
            subs.insert(channel.clone(), tx);
        }

        let msg = serde_json::json!({
            "method": "public/subscribe",
            "params": {
                "channels": [channel]
            }
        });

        self.client.send_json(msg)?;

        // Spawn callback task
        tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                // Parses into a json value initally
                let parsed_msg: MmNotification = match serde_json::from_str(&msg) {
                    Ok(m) => m,
                    Err(e) => {
                        warn!("Failed to parse channel message: {e}; raw: {msg}");
                        continue;
                    }
                };
                callback(parsed_msg.notification);
            }
        });

        info!("Subscribed to channel: {channel}");
        Ok(())
    }

    pub async fn account_orders<F>(&self, mut callback: F) -> Result<(), Error>
    where
        F: FnMut(Account) + Send + 'static,
    {
        let channel = "account_orders.".to_string();

        // Per-subscription channel from core -> user callback
        let (tx, mut rx) = mpsc::unbounded_channel::<String>();

        {
            let mut subs = self.client.subscriptions.lock().await;
            subs.insert(channel.clone(), tx);
        }

        let msg = serde_json::json!({
            "method": "public/subscribe",
            "params": {
                "channels": [channel]
            }
        });

        self.client.send_json(msg)?;

        // Spawn callback task
        tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                // Parses into a json value initally
                let parsed_msg: AccountNotification = match serde_json::from_str(&msg) {
                    Ok(m) => m,
                    Err(e) => {
                        warn!("Failed to parse channel message: {e}; raw: {msg}");
                        continue;
                    }
                };
                callback(parsed_msg.notification);
            }
        });

        info!("Subscribed to channel: {channel}");
        Ok(())
    }

    pub async fn account_persistent_orders<F>(&self, mut callback: F) -> Result<(), Error>
    where
        F: FnMut(Account) + Send + 'static,
    {
        let channel = "account_persistent_orders.".to_string();

        // Per-subscription channel from core -> user callback
        let (tx, mut rx) = mpsc::unbounded_channel::<String>();

        {
            let mut subs = self.client.subscriptions.lock().await;
            subs.insert(channel.clone(), tx);
        }

        let msg = serde_json::json!({
            "method": "public/subscribe",
            "params": {
                "channels": [channel]
            }
        });

        self.client.send_json(msg)?;

        // Spawn callback task
        tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                // Parses into a json value initally
                let parsed_msg: AccountNotification = match serde_json::from_str(&msg) {
                    Ok(m) => m,
                    Err(e) => {
                        warn!("Failed to parse channel message: {e}; raw: {msg}");
                        continue;
                    }
                };
                callback(parsed_msg.notification);
            }
        });

        info!("Subscribed to channel: {channel}");
        Ok(())
    }

    pub async fn session_orders<F>(&self, mut callback: F) -> Result<(), Error>
    where
        F: FnMut(Session) + Send + 'static,
    {
        let channel = "session_orders.".to_string();

        // Per-subscription channel from core -> user callback
        let (tx, mut rx) = mpsc::unbounded_channel::<String>();

        {
            let mut subs = self.client.subscriptions.lock().await;
            subs.insert(channel.clone(), tx);
        }

        let msg = serde_json::json!({
            "method": "public/subscribe",
            "params": {
                "channels": [channel]
            }
        });

        self.client.send_json(msg)?;

        // Spawn callback task
        tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                // Parses into a json value initally
                let parsed_msg: SessionNotification = match serde_json::from_str(&msg) {
                    Ok(m) => m,
                    Err(e) => {
                        warn!("Failed to parse channel message: {e}; raw: {msg}");
                        continue;
                    }
                };
                callback(parsed_msg.notification);
            }
        });

        info!("Subscribed to channel: {channel}");
        Ok(())
    }

    pub async fn account_trade_history<F>(&self, mut callback: F) -> Result<(), Error>
    where
        F: FnMut(Account) + Send + 'static,
    {
        let channel = "account_trade_history.".to_string();

        // Per-subscription channel from core -> user callback
        let (tx, mut rx) = mpsc::unbounded_channel::<String>();

        {
            let mut subs = self.client.subscriptions.lock().await;
            subs.insert(channel.clone(), tx);
        }

        let msg = serde_json::json!({
            "method": "public/subscribe",
            "params": {
                "channels": [channel]
            }
        });

        self.client.send_json(msg)?;

        // Spawn callback task
        tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                // Parses into a json value initally
                let parsed_msg: AccountNotification = match serde_json::from_str(&msg) {
                    Ok(m) => m,
                    Err(e) => {
                        warn!("Failed to parse channel message: {e}; raw: {msg}");
                        continue;
                    }
                };
                callback(parsed_msg.notification);
            }
        });

        info!("Subscribed to channel: {channel}");
        Ok(())
    }

    pub async fn account_order_history<F>(&self, mut callback: F) -> Result<(), Error>
    where
        F: FnMut(Account) + Send + 'static,
    {
        let channel = "account_order_history.".to_string();

        // Per-subscription channel from core -> user callback
        let (tx, mut rx) = mpsc::unbounded_channel::<String>();

        {
            let mut subs = self.client.subscriptions.lock().await;
            subs.insert(channel.clone(), tx);
        }

        let msg = serde_json::json!({
            "method": "public/subscribe",
            "params": {
                "channels": [channel]
            }
        });

        self.client.send_json(msg)?;

        // Spawn callback task
        tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                // Parses into a json value initally
                let parsed_msg: AccountNotification = match serde_json::from_str(&msg) {
                    Ok(m) => m,
                    Err(e) => {
                        warn!("Failed to parse channel message: {e}; raw: {msg}");
                        continue;
                    }
                };
                callback(parsed_msg.notification);
            }
        });

        info!("Subscribed to channel: {channel}");
        Ok(())
    }

    pub async fn account_portfolio<F>(&self, mut callback: F) -> Result<(), Error>
    where
        F: FnMut(Account) + Send + 'static,
    {
        let channel = "account_portfolio.".to_string();

        // Per-subscription channel from core -> user callback
        let (tx, mut rx) = mpsc::unbounded_channel::<String>();

        {
            let mut subs = self.client.subscriptions.lock().await;
            subs.insert(channel.clone(), tx);
        }

        let msg = serde_json::json!({
            "method": "public/subscribe",
            "params": {
                "channels": [channel]
            }
        });

        self.client.send_json(msg)?;

        // Spawn callback task
        tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                // Parses into a json value initally
                let parsed_msg: AccountNotification = match serde_json::from_str(&msg) {
                    Ok(m) => m,
                    Err(e) => {
                        warn!("Failed to parse channel message: {e}; raw: {msg}");
                        continue;
                    }
                };
                callback(parsed_msg.notification);
            }
        });

        info!("Subscribed to channel: {channel}");
        Ok(())
    }

    pub async fn account_summary<F>(&self, mut callback: F) -> Result<(), Error>
    where
        F: FnMut(Account) + Send + 'static,
    {
        let channel = "account_summary.".to_string();

        // Per-subscription channel from core -> user callback
        let (tx, mut rx) = mpsc::unbounded_channel::<String>();

        {
            let mut subs = self.client.subscriptions.lock().await;
            subs.insert(channel.clone(), tx);
        }

        let msg = serde_json::json!({
            "method": "public/subscribe",
            "params": {
                "channels": [channel]
            }
        });

        self.client.send_json(msg)?;

        // Spawn callback task
        tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                // Parses into a json value initally
                let parsed_msg: AccountNotification = match serde_json::from_str(&msg) {
                    Ok(m) => m,
                    Err(e) => {
                        warn!("Failed to parse channel message: {e}; raw: {msg}");
                        continue;
                    }
                };
                callback(parsed_msg.notification);
            }
        });

        info!("Subscribed to channel: {channel}");
        Ok(())
    }

    pub async fn account_rfqs<F>(&self, mut callback: F) -> Result<(), Error>
    where
        F: FnMut(Account) + Send + 'static,
    {
        let channel = "account_rfqs.".to_string();

        // Per-subscription channel from core -> user callback
        let (tx, mut rx) = mpsc::unbounded_channel::<String>();

        {
            let mut subs = self.client.subscriptions.lock().await;
            subs.insert(channel.clone(), tx);
        }

        let msg = serde_json::json!({
            "method": "public/subscribe",
            "params": {
                "channels": [channel]
            }
        });

        self.client.send_json(msg)?;

        // Spawn callback task
        tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                // Parses into a json value initally
                let parsed_msg: AccountNotification = match serde_json::from_str(&msg) {
                    Ok(m) => m,
                    Err(e) => {
                        warn!("Failed to parse channel message: {e}; raw: {msg}");
                        continue;
                    }
                };
                callback(parsed_msg.notification);
            }
        });

        info!("Subscribed to channel: {channel}");
        Ok(())
    }

    pub async fn account_rfq_history<F>(&self, mut callback: F) -> Result<(), Error>
    where
        F: FnMut(Account) + Send + 'static,
    {
        let channel = "account_rfq_history.".to_string();

        // Per-subscription channel from core -> user callback
        let (tx, mut rx) = mpsc::unbounded_channel::<String>();

        {
            let mut subs = self.client.subscriptions.lock().await;
            subs.insert(channel.clone(), tx);
        }

        let msg = serde_json::json!({
            "method": "public/subscribe",
            "params": {
                "channels": [channel]
            }
        });

        self.client.send_json(msg)?;

        // Spawn callback task
        tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                // Parses into a json value initally
                let parsed_msg: AccountNotification = match serde_json::from_str(&msg) {
                    Ok(m) => m,
                    Err(e) => {
                        warn!("Failed to parse channel message: {e}; raw: {msg}");
                        continue;
                    }
                };
                callback(parsed_msg.notification);
            }
        });

        info!("Subscribed to channel: {channel}");
        Ok(())
    }

    pub async fn account_conditional_orders<F>(&self, mut callback: F) -> Result<(), Error>
    where
        F: FnMut(Account) + Send + 'static,
    {
        let channel = "account_conditional_orders.".to_string();

        // Per-subscription channel from core -> user callback
        let (tx, mut rx) = mpsc::unbounded_channel::<String>();

        {
            let mut subs = self.client.subscriptions.lock().await;
            subs.insert(channel.clone(), tx);
        }

        let msg = serde_json::json!({
            "method": "public/subscribe",
            "params": {
                "channels": [channel]
            }
        });

        self.client.send_json(msg)?;

        // Spawn callback task
        tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                // Parses into a json value initally
                let parsed_msg: AccountNotification = match serde_json::from_str(&msg) {
                    Ok(m) => m,
                    Err(e) => {
                        warn!("Failed to parse channel message: {e}; raw: {msg}");
                        continue;
                    }
                };
                callback(parsed_msg.notification);
            }
        });

        info!("Subscribed to channel: {channel}");
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
        let (tx, mut rx) = mpsc::unbounded_channel::<String>();

        {
            let mut subs = self.client.subscriptions.lock().await;
            subs.insert(channel.clone(), tx);
        }

        let msg = serde_json::json!({
            "method": "public/subscribe",
            "params": {
                "channels": [channel]
            }
        });

        self.client.send_json(msg)?;

        // Spawn callback task
        tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                // Parses into a json value initally
                let parsed_msg: BasePriceNotification = match serde_json::from_str(&msg) {
                    Ok(m) => m,
                    Err(e) => {
                        warn!("Failed to parse channel message: {e}; raw: {msg}");
                        continue;
                    }
                };
                callback(parsed_msg.notification);
            }
        });

        info!("Subscribed to channel: {channel}");
        Ok(())
    }

    pub async fn instruments<F>(&self, mut callback: F) -> Result<(), Error>
    where
        F: FnMut(InstrumentsPayload) + Send + 'static,
    {
        let channel = "instruments.".to_string();

        // Per-subscription channel from core -> user callback
        let (tx, mut rx) = mpsc::unbounded_channel::<String>();

        {
            let mut subs = self.client.subscriptions.lock().await;
            subs.insert(channel.clone(), tx);
        }

        let msg = serde_json::json!({
            "method": "public/subscribe",
            "params": {
                "channels": [channel]
            }
        });

        self.client.send_json(msg)?;

        // Spawn callback task
        tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                // Parses into a json value initally
                let parsed_msg: InstrumentsPayloadNotification = match serde_json::from_str(&msg) {
                    Ok(m) => m,
                    Err(e) => {
                        warn!("Failed to parse channel message: {e}; raw: {msg}");
                        continue;
                    }
                };
                callback(parsed_msg.notification);
            }
        });

        info!("Subscribed to channel: {channel}");
        Ok(())
    }

    pub async fn rfqs<F>(&self, mut callback: F) -> Result<(), Error>
    where
        F: FnMut(RfqsPayload) + Send + 'static,
    {
        let channel = "rfqs.".to_string();

        // Per-subscription channel from core -> user callback
        let (tx, mut rx) = mpsc::unbounded_channel::<String>();

        {
            let mut subs = self.client.subscriptions.lock().await;
            subs.insert(channel.clone(), tx);
        }

        let msg = serde_json::json!({
            "method": "public/subscribe",
            "params": {
                "channels": [channel]
            }
        });

        self.client.send_json(msg)?;

        // Spawn callback task
        tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                // Parses into a json value initally
                let parsed_msg: RfqsPayloadNotification = match serde_json::from_str(&msg) {
                    Ok(m) => m,
                    Err(e) => {
                        warn!("Failed to parse channel message: {e}; raw: {msg}");
                        continue;
                    }
                };
                callback(parsed_msg.notification);
            }
        });

        info!("Subscribed to channel: {channel}");
        Ok(())
    }

    pub async fn index_components<F>(&self, underlying: &str, mut callback: F) -> Result<(), Error>
    where
        F: FnMut(IndexComponents) + Send + 'static,
    {
        let channel = format!("index_components.{underlying}");

        // Per-subscription channel from core -> user callback
        let (tx, mut rx) = mpsc::unbounded_channel::<String>();

        {
            let mut subs = self.client.subscriptions.lock().await;
            subs.insert(channel.clone(), tx);
        }

        let msg = serde_json::json!({
            "method": "public/subscribe",
            "params": {
                "channels": [channel]
            }
        });

        self.client.send_json(msg)?;

        // Spawn callback task
        tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                // Parses into a json value initally
                let parsed_msg: IndexComponentsNotification = match serde_json::from_str(&msg) {
                    Ok(m) => m,
                    Err(e) => {
                        warn!("Failed to parse channel message: {e}; raw: {msg}");
                        continue;
                    }
                };
                callback(parsed_msg.notification);
            }
        });

        info!("Subscribed to channel: {channel}");
        Ok(())
    }
}
