use druid::{
    im::{vector, Vector},
    Data, Lens,
};
use std::{
    rc::Rc,
    str::FromStr,
    sync::{Arc, Mutex},
};
use uuid::Uuid;

use tokio_tungstenite::tungstenite::Message;

use secp256k1::{rand::rngs::OsRng, schnorrsig, Secp256k1, SecretKey};

use crate::data::{contact::Contact, conversation::Conversation};
use futures_channel::mpsc::{self, UnboundedSender};

use super::{config::Config, user::User};

#[derive(Clone)]
pub struct ChatMsg {
    pub receiver_pk: String,
    pub content: String,
}

impl ChatMsg {
    pub fn new(receiver_pk: &str, content: &str) -> Self {
        Self {
            receiver_pk: receiver_pk.into(),
            content: content.into(),
        }
    }
}

#[derive(Data, Clone, Lens)]
pub struct AppState {
    pub chat_messages: Vector<String>,
    pub msg_to_send: String,
    pub tx: Arc<Mutex<Option<UnboundedSender<Message>>>>,
    pub config: Config,
    pub user: User,
    pub new_contact_alias: String,
    pub new_contact_pk: String,
    pub current_chat_contact: Contact,
    pub conversations: Vector<Rc<Conversation>>,
    pub selected_conv: Option<Rc<Conversation>>,
    sub_id: String,
}

impl AppState {
    pub fn new() -> Self {
        let config = Config::load();
        let conversations = config
            .contacts
            .clone()
            .iter_mut()
            .map(|contact| Rc::new(Conversation::new(contact.clone())))
            .collect();

        Self {
            chat_messages: vector!(),
            msg_to_send: "".into(),
            tx: Arc::new(Mutex::new(None)),
            new_contact_pk: "".into(),
            new_contact_alias: "".into(),
            current_chat_contact: Contact::new("", ""),
            conversations,
            selected_conv: None,
            sub_id: "".into(),
            config,
            user: User::new("", ""),
        }
    }

    pub fn get_authors(&self) -> Vec<String> {
        self.config
            .contacts
            .clone()
            .into_iter()
            .map(|c| c.pk)
            .collect()
    }

    pub fn gen_sub_id(&mut self) -> String {
        let id = Uuid::new_v4().to_string();
        self.sub_id = id.clone();
        id
    }

    pub fn push_new_msg(&mut self, new_msg: ChatMsg) {
        self.chat_messages.push_front(new_msg.content);
    }

    pub fn generate_sk(&mut self) {
        self.user.generate_keys_from_sk();
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
        for conv in self.conversations.iter() {
            if conv.contact.pk == pk {
                self.selected_conv = Some(Rc::clone(conv));
            }
        }

        println!("{:?}", self.selected_conv.as_ref().unwrap());
    }

    pub fn add_contact(&mut self) {
        self.config
            .add_contact(&Contact::new(&self.new_contact_alias, &self.new_contact_pk));
        self.new_contact_alias = "".into();
        self.new_contact_pk = "".into();
    }

    pub fn delete_contact(&mut self, pk: &str) {
        self.config.delete_contact(&pk);
    }
}
