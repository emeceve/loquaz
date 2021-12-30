use std::str::FromStr;

use nostr::Event;
use secp256k1::schnorrsig::PublicKey;
use thiserror::Error;
use tokio::sync::{
    mpsc::{self, Receiver, Sender},
    oneshot,
};

use super::{
    config::provider::ConfigProvider,
    entities::{
        config::Config,
        contact::Contact,
        relay_pool::RelayPool,
        subscription::{self, Subscription},
    },
};

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
}

#[derive(Clone)]
pub struct CoreTaskHandle {
    sender: Sender<CoreTaskMessage>,
}

impl CoreTaskHandle {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel(32);
        let core_task = CoreTask::new(receiver);
        tokio::spawn(start_core_task(core_task));
        Self { sender }
    }

    pub async fn add_relay(&self, url: String) -> CoreTaskHandleEvent {
        let (tx, rx) = oneshot::channel();
        self.sender
            .send(CoreTaskMessage::AddRelay { url, res_to: tx })
            .await;
        CoreTaskHandleEvent::RelayAdded(
            rx.await.unwrap_or(Err(CoreTaskHandleError::AddRelayFailed)),
        )
    }

    pub async fn remove_relay(&self, url: String) -> CoreTaskHandleEvent {
        let (tx, rx) = oneshot::channel();
        self.sender
            .send(CoreTaskMessage::RemoveRelay { url, res_to: tx })
            .await;
        CoreTaskHandleEvent::RemovedRelay(
            rx.await
                .unwrap_or(Err(CoreTaskHandleError::RemoveRelayFailed)),
        )
    }

    pub async fn connect_relay(&self, url: String) -> CoreTaskHandleEvent {
        let (tx, rx) = oneshot::channel();
        self.sender
            .send(CoreTaskMessage::ConnectRelay { url, res_to: tx })
            .await;
        CoreTaskHandleEvent::RemovedRelay(
            rx.await
                .unwrap_or(Err(CoreTaskHandleError::RemoveRelayFailed)),
        )
    }
    pub async fn disconnect_relay(&self, url: String) -> CoreTaskHandleEvent {
        let (tx, rx) = oneshot::channel();
        self.sender
            .send(CoreTaskMessage::DisconnectRelay { url, res_to: tx })
            .await;
        CoreTaskHandleEvent::RemovedRelay(
            rx.await
                .unwrap_or(Err(CoreTaskHandleError::RemoveRelayFailed)),
        )
    }
    pub async fn subscribe(&self, pk: String) -> CoreTaskHandleEvent {
        let (tx, rx) = oneshot::channel();
        self.sender
            .send(CoreTaskMessage::SubscribeInRelays { pk, res_to: tx })
            .await;
        CoreTaskHandleEvent::RemovedRelay(
            rx.await
                .unwrap_or(Err(CoreTaskHandleError::RemoveRelayFailed)),
        )
    }
    pub async fn add_contact(&self, contact: Contact) -> CoreTaskHandleEvent {
        let (tx, rx) = oneshot::channel();
        self.sender
            .send(CoreTaskMessage::AddContact {
                new_contact: contact,
                res_to: tx,
            })
            .await;
        CoreTaskHandleEvent::ContactAdded(
            rx.await
                .unwrap_or(Err(CoreTaskHandleError::AddContactFailed)),
        )
    }

    pub async fn remove_contact(&self, contact: Contact) -> CoreTaskHandleEvent {
        let (tx, rx) = oneshot::channel();
        self.sender
            .send(CoreTaskMessage::RemoveContact {
                contact,
                res_to: tx,
            })
            .await;
        CoreTaskHandleEvent::RemovedContact(
            rx.await
                .unwrap_or(Err(CoreTaskHandleError::RemoveContactFailed)),
        )
    }

    pub async fn load_configs(&self) -> CoreTaskHandleEvent {
        let (tx, rx) = oneshot::channel();
        self.sender
            .send(CoreTaskMessage::LoadConfig { res_to: tx })
            .await;

        CoreTaskHandleEvent::ConfigLoaded(rx.await.unwrap())
    }
}

//####### Core Task #########

enum CoreTaskMessage {
    AddRelay {
        url: String,
        res_to: oneshot::Sender<Result<(), CoreTaskHandleError>>,
    },
    RemoveRelay {
        url: String,
        res_to: oneshot::Sender<Result<(), CoreTaskHandleError>>,
    },
    ConnectRelay {
        url: String,
        res_to: oneshot::Sender<Result<(), CoreTaskHandleError>>,
    },
    DisconnectRelay {
        url: String,
        res_to: oneshot::Sender<Result<(), CoreTaskHandleError>>,
    },
    SubscribeInRelays {
        pk: String,
        res_to: oneshot::Sender<Result<(), CoreTaskHandleError>>,
    },
    AddContact {
        new_contact: Contact,
        res_to: oneshot::Sender<Result<(), CoreTaskHandleError>>,
    },
    RemoveContact {
        contact: Contact,
        res_to: oneshot::Sender<Result<(), CoreTaskHandleError>>,
    },
    LoadConfig {
        res_to: oneshot::Sender<Result<Config, CoreTaskHandleError>>,
    },
}

struct CoreTask {
    receiver: Receiver<CoreTaskMessage>,
    config: ConfigProvider,
    relay_pool: RelayPool,
}

impl CoreTask {
    fn new(receiver: Receiver<CoreTaskMessage>) -> Self {
        let config = ConfigProvider::load();
        let mut relay_pool = RelayPool::new();

        for (k, v) in config.get().relays_url {
            relay_pool.add(&v);
        }

        Self {
            receiver,
            config,
            relay_pool,
        }
    }

    async fn handle_message(&mut self, msg: CoreTaskMessage) {
        match msg {
            CoreTaskMessage::AddRelay { url, res_to } => {
                self.relay_pool.add(&url);
                res_to.send(
                    self.config
                        .add_relay(url)
                        .map_err(|_| CoreTaskHandleError::AddRelayFailed),
                );
            }
            CoreTaskMessage::RemoveRelay { url, res_to } => {
                res_to.send(
                    self.config
                        .remove_relay(&url)
                        .map_err(|_| CoreTaskHandleError::AddRelayFailed),
                );
            }
            CoreTaskMessage::ConnectRelay { url, res_to } => {
                self.relay_pool.connect_relay(&url).await;
                res_to.send(Ok(()));
            }
            CoreTaskMessage::DisconnectRelay { url, res_to } => {
                self.relay_pool.disconnect_relay(&url).await;
                res_to.send(Ok(()));
            }
            CoreTaskMessage::SubscribeInRelays { pk, res_to } => {
                let authors: Vec<PublicKey> = self
                    .config
                    .get()
                    .contacts
                    .clone()
                    .into_iter()
                    .map(|(k, _)| PublicKey::from_str(&k).unwrap())
                    .collect();
                let filters = vec![nostr::SubscriptionFilter::new()
                    .authors(authors)
                    .kind(nostr::Kind::EncryptedDirectMessage)
                    .tag_p(PublicKey::from_str(&pk).unwrap())];

                self.relay_pool.start_sub(filters).await;
                res_to.send(Ok(()));
            }
            CoreTaskMessage::AddContact {
                new_contact,
                res_to,
            } => {
                res_to.send(
                    self.config
                        .add_contact(new_contact)
                        .map_err(|_| CoreTaskHandleError::AddRelayFailed),
                );
            }
            CoreTaskMessage::RemoveContact { contact, res_to } => {
                res_to.send(
                    self.config
                        .remove_contact(contact)
                        .map_err(|_| CoreTaskHandleError::RemoveContactFailed),
                );
            }
            CoreTaskMessage::LoadConfig { res_to } => {
                res_to.send(Ok(self.config.get()));
            }
        }
    }
}

async fn start_core_task(mut task: CoreTask) {
    while let Some(msg) = task.receiver.recv().await {
        task.handle_message(msg).await;
    }
}
