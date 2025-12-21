use crate::channels::namespaces::accounting::AccountingSubscriptions;
use crate::channels::namespaces::conditional::ConditionalSubscriptions;
use crate::channels::namespaces::market_data::MarketDataSubscriptions;
use crate::channels::namespaces::mm_prot::MmProtSubscriptions;
use crate::channels::namespaces::mm_rfq::MmRfqSubscriptions;
use crate::ws_client::WsClient;

pub struct Subscriptions<'a> {
    pub client: &'a WsClient,
}
impl<'a> Subscriptions<'a> {
    // We pull out the namespaces here for easier access
    pub fn market_data(&self) -> MarketDataSubscriptions<'a> {
        MarketDataSubscriptions {
            client: self.client,
        }
    }
    pub fn accounting(&self) -> AccountingSubscriptions<'a> {
        AccountingSubscriptions {
            client: self.client,
        }
    }
    pub fn conditional(&self) -> ConditionalSubscriptions<'a> {
        ConditionalSubscriptions {
            client: self.client,
        }
    }
    pub fn mm_prot(&self) -> MmProtSubscriptions<'a> {
        MmProtSubscriptions {
            client: self.client,
        }
    }
    pub fn mm_rfq(&self) -> MmRfqSubscriptions<'a> {
        MmRfqSubscriptions {
            client: self.client,
        }
    }
}
