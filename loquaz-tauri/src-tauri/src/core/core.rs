use super::{
    config::{Config, ConfigProvider, Contact},
    conversations::{Conversation, Conversations, ConvsNotifications},
    relay_pool::{RelayPool, RelayPoolNotifications},
    user::User,
};
use log::debug;
use nostr::Event;
use secp256k1::schnorrsig::PublicKey;
use std::{
    fmt::Debug,
    sync::{Arc, Mutex},
};
use thiserror::Error;
use tokio::sync::broadcast;

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
    pub relay_pool: RelayPool,
    conversations: Arc<Mutex<Conversations>>,
    user: Arc<Mutex<User>>,
}

impl CoreTaskHandle {
    pub fn new() -> Self {
        let config = ConfigProvider::load();
        let mut relay_pool = RelayPool::new();
        let conversations = Arc::new(Mutex::new(Conversations::new()));
        let user = Arc::new(Mutex::new(User::new()));

        for url in config.list_relays_url() {
            relay_pool.add(&url);
        }

        for c in config.list_contacts() {
            conversations.lock().unwrap().add_conv(Conversation::new(c));
        }

        let mut rec_ch = relay_pool.get_notifications_ch();
        let conversations_clone = conversations.clone();
        let user_clone = user.clone();
        tokio::spawn(async move {
            while let Ok(noti) = rec_ch.recv().await {
                debug!("Received from broadcast {:?}", noti);
                match noti {
                    RelayPoolNotifications::ReceivedEvent { ev } => {
                        conversations_clone
                            .lock()
                            .unwrap()
                            .try_add_message_from_ev(ev, &user_clone.lock().unwrap());
                    }
                    _ => (),
                };
            }
        });

        Self {
            config,
            relay_pool,
            conversations,
            user,
        }
    }

    pub fn get_convs_notifications(&self) -> broadcast::Receiver<ConvsNotifications> {
        self.conversations.lock().unwrap().get_notifications_ch()
    }

    pub fn get_conv(&self, pk: String) -> Option<Conversation> {
        Some(
            self.conversations
                .lock()
                .unwrap()
                .get_conv(&pk)
                .unwrap()
                .clone(),
        )
    }

    pub async fn send_msg_to_contact(&mut self, contact_pk: &str, content: &str) {
        let mut send_to_relays = false;
        let mut new_ev = None;

        let user = self.user.lock().unwrap().clone();
        if let Ok(ev) = Event::new_encrypted_direct_msg(
            &user.keys,
            &nostr::Keys::new_pub_only(&contact_pk.to_string()).unwrap(),
            content,
        ) {
            self.conversations
                .lock()
                .unwrap()
                .try_add_message_from_ev(ev.clone(), &user);
            send_to_relays = true;
            new_ev = Some(ev);
        } else {
            dbg!("Error while creating event");
        }

        //This is necessary because we cant send a mutex to another thread
        //and Tokio runtime can move this task between threads at every .await
        //https://tokio.rs/tokio/tutorial/shared-state
        if send_to_relays && new_ev.is_some() {
            self.relay_pool.send_ev(new_ev.unwrap()).await;
        }
    }

    pub fn get_noti_ch(&self) -> broadcast::Receiver<RelayPoolNotifications> {
        return self.relay_pool.get_notifications_ch();
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

    pub async fn subscribe(&mut self) {
        let authors: Vec<PublicKey> = self
            .config
            .list_contacts()
            .into_iter()
            .map(|c| c.pk.to_owned())
            .collect();
        let user_pk = self.user.lock().unwrap().get_pk();
        //Subscribe to DM events whose authors are in
        //contact list and is intended to user PK
        let filter_contacts_dm_events = nostr::SubscriptionFilter::new()
            .authors(authors.clone())
            .kind(nostr::Kind::EncryptedDirectMessage)
            .pubkey(user_pk.clone());

        let mut filters = vec![filter_contacts_dm_events];
        //Subscribe to DM events whose author is the user
        //and is intended to its contacts
        //
        //TODO: Change this to use pubkey as a vector. Needs update
        // nostr-rs
        for a in authors.iter() {
            filters.push(
                nostr::SubscriptionFilter::new()
                    .authors(vec![user_pk.clone()])
                    .kind(nostr::Kind::EncryptedDirectMessage)
                    .pubkey(a.clone()),
            );
        }

        self.relay_pool.start_sub(filters).await;
    }
    pub async fn add_contact(&mut self, contact: Contact) {
        self.config.add_contact(contact.clone());
        self.conversations
            .lock()
            .unwrap()
            .add_conv(Conversation::new(contact));
        //Update filters and resubscribe based on updated conversations
        self.subscribe().await;
    }

    pub async fn remove_contact(&mut self, contact: Contact) {
        self.config.remove_contact(contact.clone());
        self.conversations
            .lock()
            .unwrap()
            .remove_conv(&contact.pk.to_string());

        self.relay_pool.remove_contact_events(contact).await;
        //Update filters and resubscribe based on updated conversations
        self.subscribe().await;
    }

    pub fn get_config(&self) -> (Vec<String>, Vec<Contact>) {
        (self.config.list_relays_url(), self.config.list_contacts())
    }

    pub fn import_user_sk(&self, sk: String) {
        self.user.lock().unwrap().import_sk(&sk);
    }

    pub fn gen_new_user_keypair(&mut self) {
        *self.user.lock().unwrap() = User::new();
    }

    pub fn get_user(&self) -> User {
        (*self.user.lock().unwrap()).clone()
    }
}
