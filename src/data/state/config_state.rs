use crate::core::entities::config::Config;

use super::contact_state::ContactState;
use druid::{
    im::{HashMap, Vector},
    Data, Lens,
};

#[derive(Clone, Data, Lens)]
pub struct ConfigState {
    pub contacts: Vector<ContactState>,
    pub relays_url: Vector<String>,
}

impl ConfigState {
    pub fn new() -> Self {
        Self {
            contacts: Vector::new(),
            relays_url: Vector::new(),
        }
    }

    pub fn from_entity(config_entity: &Config) -> ConfigState {
        let mut contacts = Vector::new();
        for (k, v) in config_entity.clone().contacts {
            contacts.push_back(ContactState::from_entity(&v));
        }
        ConfigState {
            contacts,
            relays_url: config_entity
                .relays_url
                .clone()
                .into_iter()
                .map(|(k, v)| k)
                .collect(),
        }
    }
}
