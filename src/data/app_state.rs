use druid::{
    im::{vector, HashMap, Vector},
    Data, Lens,
};
use secp256k1::schnorrsig::PublicKey;
use std::{
    rc::Rc,
    str::FromStr,
    sync::{Arc, Mutex},
};
use uuid::Uuid;

use tokio_tungstenite::tungstenite::Message;

use crate::{broker::BrokerEvent, core::config::Contact};
use futures_channel::mpsc::{self, UnboundedSender};

use super::{
    state::{
        config_state::ConfigState,
        contact_state::ContactState,
        conversation_state::{ChatMsgState, ConversationState, MsgState},
    },
    user::User,
};

// TODO: look into druid's Arcstr because we can use that for some of these
#[derive(Data, Clone, Lens)]
pub struct AppState {
    pub chat_messages: Vector<String>,
    pub msg_to_send: String,
    pub tx: Arc<Mutex<Option<UnboundedSender<Message>>>>,
    pub sender_broker: Arc<Option<tokio::sync::mpsc::Sender<BrokerEvent>>>,
    pub config: ConfigState,
    pub user: User,
    pub new_contact_alias: String,
    pub new_contact_pk: String,
    pub new_relay_ulr: String,
    pub current_chat_contact: ContactState,
    pub conversations: HashMap<String, ConversationState>,
    pub selected_conv: Option<ConversationState>,
    sub_id: String,
}

impl AppState {
    pub fn new() -> Self {
        let config = ConfigState::new();
        let mut conversations = HashMap::new();
        for i in config.contacts.iter() {
            conversations.insert(i.pk.clone(), ConversationState::new(i.clone()));
        }
        Self {
            chat_messages: vector!(),
            msg_to_send: "".into(),
            tx: Arc::new(Mutex::new(None)),
            sender_broker: Arc::new(None),
            new_contact_pk: "".into(),
            new_contact_alias: "".into(),
            new_relay_ulr: "".into(),
            current_chat_contact: ContactState::new("", ""),
            conversations,
            selected_conv: None,
            sub_id: "".into(),
            config,
            user: User::new("", ""),
        }
    }

    //TODO: use PublicKey in contact
    pub fn get_authors(&self) -> Vec<PublicKey> {
        self.config
            .contacts
            .clone()
            .into_iter()
            .map(|c| PublicKey::from_str(&c.pk).unwrap())
            .collect()
    }

    pub fn gen_sub_id(&mut self) -> String {
        let id = Uuid::new_v4().to_string();
        self.sub_id = id.clone();
        id
    }

    pub fn push_conv_msg(&mut self, msg: &MsgState, conversation_pk: &str) {
        match self.conversations.get_mut(conversation_pk) {
            Some(conv) => conv.push_msg(msg),
            None => println!("Conversation not found!"),
        }
        if self.selected_conv.is_some() {
            self.selected_conv.as_mut().unwrap().push_msg(msg);
        }
    }

    pub fn push_new_msg(&mut self, new_msg: ChatMsgState) {
        self.chat_messages.push_front(new_msg.content);
    }

    pub fn generate_sk(&mut self) {
        self.user.generate_keys_from_sk();
        let sender = (*self.sender_broker).clone();
        let pk = self.user.pk.clone();
        tokio::spawn(async move {
            sender
                .unwrap()
                .send(crate::broker::BrokerEvent::SubscribeInRelays { pk })
                .await;
        });
    }

    pub fn set_current_chat(&mut self, pk: &str) {
        for contact in self.config.contacts.iter() {
            if contact.pk == pk {
                self.current_chat_contact = contact.clone();
            }
        }

        println!("{:?}", self.current_chat_contact);
    }
    pub fn set_conv(&mut self, pk: &str) {
        //        for conv in self.conversations.iter() {
        //            if conv.contact.pk == pk {
        //                self.selected_conv = Some(Rc::clone(conv));
        //            }
        //        }
        match self.conversations.get_mut(pk) {
            Some(conv) => self.selected_conv = Some(conv.clone()),
            None => println!("Conversation not found!"),
        }

        println!("{:?}", self.selected_conv.as_ref().unwrap());
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
