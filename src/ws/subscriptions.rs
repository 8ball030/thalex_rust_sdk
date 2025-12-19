use log::{info, warn};
use tokio::sync::mpsc;

use crate::{
    models::{Book, BookNotification, Delay, Ticker, TickerNotification},
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
}
