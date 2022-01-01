use super::{
    config::{Config, ConfigProvider, Contact},
    relay_pool::{RelayPool, RelayPoolNotifications},
};
use nostr::Event;
use secp256k1::schnorrsig::PublicKey;
use std::str::FromStr;
use thiserror::Error;
use tokio::sync::mpsc::{self, Receiver};

//####### Core Task Handle Errors  #########
#[derive(Debug, Error)]
pub enum CoreTaskHandleError {
    #[error("Adding new relay failed")]
    AddRelayFailed,
    #[error("Adding new contact failed")]
    AddContactFailed,
    #[error("Removing new relay failed")]
    RemoveRelayFailed,
    #[error("Removing new contact failed")]
    RemoveContactFailed,
}

//####### Core Task Handle  #########

#[derive(Debug)]
pub enum CoreTaskHandleEvent {
    ReceivedEvent { ev: Event },
    RelayAdded(Result<(), CoreTaskHandleError>),
    RemovedRelay(Result<(), CoreTaskHandleError>),
    ContactAdded(Result<(), CoreTaskHandleError>),
    RemovedContact(Result<(), CoreTaskHandleError>),
    ConfigLoaded(Result<Config, CoreTaskHandleError>),
    Initiated,
}

pub struct CoreTaskHandle {
    config: ConfigProvider,
    relay_pool: RelayPool,
    receiver_pool_notifications: Receiver<RelayPoolNotifications>,
}

impl CoreTaskHandle {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel(64);
        let config = ConfigProvider::load();
        let mut relay_pool = RelayPool::new(sender);

        for url in config.list_relays_url() {
            relay_pool.add(&url);
        }

        Self {
            config,
            relay_pool,
            receiver_pool_notifications: receiver,
        }
    }

    pub async fn add_relay(&mut self, url: String) -> CoreTaskHandleEvent {
        self.relay_pool.add(&url);
        self.config
            .add_relay(url)
            .map_err(|_| CoreTaskHandleError::AddRelayFailed);

        CoreTaskHandleEvent::RelayAdded(Ok(()))
    }

    pub async fn remove_relay(&mut self, url: String) -> CoreTaskHandleEvent {
        self.config
            .remove_relay(&url)
            .map_err(|_| CoreTaskHandleError::AddRelayFailed);
        CoreTaskHandleEvent::RemovedRelay(Ok(()))
    }

    pub async fn connect_relay(&mut self, url: String) {
        self.relay_pool.connect_relay(&url).await;
    }
    pub async fn disconnect_relay(&mut self, url: String) {
        self.relay_pool.disconnect_relay(&url).await;
    }
    pub async fn subscribe(&mut self, pk: String) {
        let authors: Vec<PublicKey> = self
            .config
            .list_contacts()
            .into_iter()
            .map(|c| c.pk.to_owned())
            .collect();
        let filters = vec![nostr::SubscriptionFilter::new()
            .authors(authors)
            .kind(nostr::Kind::EncryptedDirectMessage)
            .tag_p(PublicKey::from_str(&pk).unwrap())];

        self.relay_pool.start_sub(filters).await;
    }
    pub async fn add_contact(&mut self, contact: Contact) {
        self.config.add_contact(contact);
    }

    pub async fn remove_contact(&mut self, contact: Contact) {
        self.config.remove_contact(contact);
    }

    pub fn get_config(&self) -> (Vec<String>, Vec<Contact>) {
        (self.config.list_relays_url(), self.config.list_contacts())
    }
}
