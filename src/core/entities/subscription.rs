use std::collections::HashMap;

use nostr::SubscriptionFilter;
use uuid::Uuid;

pub struct Subscription {
    filters: Vec<SubscriptionFilter>,
    channels: Vec<Channel>,
}

impl Subscription {
    pub fn new() -> Self {
        Self {
            filters: vec![],
            channels: vec![],
        }
    }

    pub fn update_filters(&mut self, filters: Vec<SubscriptionFilter>) {
        self.filters = filters;
    }

    pub fn add_channel(&mut self, channel: Channel) {
        self.channels.push(channel);
    }
}

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
