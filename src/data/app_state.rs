use druid::{
    im::{vector, HashMap, Vector},
    Data, Lens,
};
use secp256k1::schnorrsig::PublicKey;
use std::{str::FromStr, sync::Arc};

use crate::{broker::BrokerEvent, core::config::Contact};

use super::{
    router::Route,
    state::{
        config_state::ConfigState, contact_state::ContactState,
        conversation_state::ConversationState, user_state::UserState,
    },
};

// TODO: look into druid's Arcstr because we can use that for some of these
#[derive(Data, Clone, Lens)]
pub struct AppState {
    pub msg_to_send: String,
    pub sender_broker: Arc<Option<tokio::sync::mpsc::Sender<BrokerEvent>>>,
    pub config: ConfigState,
    pub user: UserState,
    pub new_contact_alias: String,
    pub new_contact_pk: String,
    pub new_relay_ulr: String,
    pub conversations: HashMap<String, ConversationState>,
    pub selected_conv: Option<ConversationState>,
    pub route: Route,
}

impl AppState {
    pub fn new() -> Self {
        let config = ConfigState::new();
        let mut conversations = HashMap::new();
        for i in config.contacts.iter() {
            conversations.insert(i.pk.clone(), ConversationState::new(i.clone(), vec![]));
        }
        Self {
            msg_to_send: "".into(),
            sender_broker: Arc::new(None),
            new_contact_pk: "".into(),
            new_contact_alias: "".into(),
            new_relay_ulr: "".into(),
            conversations,
            selected_conv: None,
            config,
            user: UserState::new("", ""),
            route: Route::Settings,
        }
    }

    pub fn restore_sk(&mut self) {
        //      let old_pk = self.user.pk.clone();
        //      self.user.restore_keys_from_sk();
        //      let pk = self.user.pk.clone();

        //      if old_pk == pk {
        //          eprintln!("that's the same pk bro")
        //      } else {
        //          let sender = (*self.sender_broker).clone();
        //          tokio::spawn(async move {
        //              sender
        //                  .unwrap()
        //                  .send(crate::broker::BrokerEvent::SubscribeInRelays { pk })
        //                  .await;
        //          });
        //      }
        //
        //
        //
        //

        let sk = self.user.sk.clone();
        let sender = (*self.sender_broker).clone();
        tokio::spawn(async move {
            sender
                .unwrap()
                .send(crate::broker::BrokerEvent::RestoreKeyPair { sk })
                .await;
        });
    }

    pub fn generate_sk(&mut self) {
        let sender = (*self.sender_broker).clone();
        tokio::spawn(async move {
            sender
                .unwrap()
                .send(crate::broker::BrokerEvent::GenerateNewKeyPair)
                .await;
        });
    }

    pub fn set_conv(&mut self, pk: &str) {
        let pk = pk.into();
        let sender = (*self.sender_broker).clone();
        tokio::spawn(async move {
            sender
                .unwrap()
                .send(crate::broker::BrokerEvent::SetConversation { pk })
                .await;
        });
    }

    pub fn add_relay_url(&mut self) {
        let sender = (*self.sender_broker).clone();
        let url_clone = String::from(&self.new_relay_ulr);
        tokio::spawn(async move {
            sender
                .unwrap()
                .send(crate::broker::BrokerEvent::AddRelay { url: url_clone })
                .await;
        });
        self.new_relay_ulr = "".into();
    }
    pub fn remove_relay(&mut self, relay_url: &str) {
        let sender = (*self.sender_broker).clone();
        let url_clone = String::from(relay_url);
        tokio::spawn(async move {
            sender
                .unwrap()
                .send(crate::broker::BrokerEvent::RemoveRelay { url: url_clone })
                .await;
        });
    }
    pub fn connect_relay(&mut self, relay_url: &str) {
        let sender = (*self.sender_broker).clone();
        let url_clone = String::from(relay_url);
        tokio::spawn(async move {
            sender
                .unwrap()
                .send(crate::broker::BrokerEvent::ConnectRelay { url: url_clone })
                .await;
        });
    }

    pub fn add_contact(&mut self) {
        let sender = (*self.sender_broker).clone();
        if let Ok(pk) = PublicKey::from_str(&self.new_contact_pk) {
            let new_contact = Contact::new(&self.new_contact_alias, pk);
            tokio::spawn(async move {
                sender
                    .unwrap()
                    .send(crate::broker::BrokerEvent::AddContact { new_contact })
                    .await;
            });
            self.new_contact_alias = "".into();
            self.new_contact_pk = "".into();
        } else {
            eprintln!("Bad public key!");
        }
    }

    pub fn delete_contact(&mut self, contact_state: &ContactState) {
        if let Ok(pk) = PublicKey::from_str(&contact_state.pk) {
            let sender = (*self.sender_broker).clone();
            let contact = Contact::new(&contact_state.alias, pk);
            tokio::spawn(async move {
                sender
                    .unwrap()
                    .send(crate::broker::BrokerEvent::RemoveContact { contact })
                    .await;
            });
        }
    }
}
