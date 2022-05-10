use std::collections::HashMap;

use nostr::SubscriptionFilter;
use uuid::Uuid;

pub struct Subscription {
    filters: Vec<SubscriptionFilter>,
    channels: HashMap<String, Channel>,
}

impl Subscription {
    pub fn new() -> Self {
        Self {
            filters: vec![],
            channels: HashMap::new(),
        }
    }

    pub fn update_filters(&mut self, filters: Vec<SubscriptionFilter>) {
        self.filters = filters;
    }

    pub fn get_filters(&self) -> Vec<SubscriptionFilter> {
        self.filters.clone()
    }

    pub fn _add_channel(&mut self, relay_url: String, channel: Channel) {
        self.channels.insert(relay_url, channel);
    }

    pub fn remove_channel(&mut self, relay_url: &str) -> Option<Channel> {
        self.channels.remove(relay_url)
    }

    pub fn get_channel(&mut self, relay_url: &str) -> Channel {
        self.channels
            .entry(relay_url.into())
            .or_insert(Channel::new(&relay_url))
            .clone()
    }
}

#[derive(Clone)]
pub struct Channel {
    pub relay_url: String,
    pub id: String,
}

impl Channel {
    pub fn new(relay_url: &str) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            relay_url: relay_url.into(),
        }
    }
}
