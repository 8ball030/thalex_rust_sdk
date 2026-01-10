use dashmap::DashMap;
use log::warn;
use serde::de::{self, MapAccess, Visitor};
use std::{fmt, sync::Arc};

use crate::types::{ResponseSender, SubscriptionChannel};

pub struct RoutingVisitor<'a> {
    pub text: &'a str,
    pub pending_requests: &'a Arc<DashMap<u64, ResponseSender>>,
    pub public_subscriptions: &'a Arc<DashMap<String, SubscriptionChannel>>,
    pub private_subscriptions: &'a Arc<DashMap<String, SubscriptionChannel>>,
}

impl<'de, 'a> Visitor<'de> for RoutingVisitor<'a> {
    type Value = ();

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("a JSON object")
    }

    #[inline(always)]
    fn visit_map<M>(self, mut map: M) -> Result<(), M::Error>
    where
        M: MapAccess<'de>,
    {
        while let Some(key) = map.next_key::<&str>()? {
            match key {
                "id" => {
                    let id: Option<u64> = map.next_value()?;
                    drain_map(&mut map)?;
                    match id {
                        None => {
                            // id: null
                            // subscription confirmation or error
                            return Ok(());
                        }
                        Some(id) => {
                            if let Some((_, tx)) = self.pending_requests.remove(&id) {
                                let _ = tx.send(self.text.into());
                            }
                            return Ok(());
                        }
                    }
                }

                "channel_name" => {
                    let channel: &str = map.next_value()?;
                    drain_map(&mut map)?;

                    for route in [self.private_subscriptions, self.public_subscriptions] {
                        if let Some(sender) = route.get_mut(channel) {
                            if sender.send(self.text.into()).is_err() {
                                route.remove(channel);
                            }
                            return Ok(());
                        }
                    }

                    warn!("No subscription handler for channel: {channel}");
                    return Ok(());
                }

                // Skip everything else fast
                _ => {
                    let _: de::IgnoredAny = map.next_value()?;
                }
            }
        }

        // No routing keys found
        warn!("Received unhandled message: {}", self.text);
        Ok(())
    }
}

fn drain_map<'de, M>(map: &mut M) -> Result<(), M::Error>
where
    M: MapAccess<'de>,
{
    while map
        .next_entry::<de::IgnoredAny, de::IgnoredAny>()?
        .is_some()
    {}
    Ok(())
}
