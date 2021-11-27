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

use crate::data::{contact::Contact, conversation::Conversation};
use futures_channel::mpsc::{self, UnboundedSender};

use super::{
    config::Config,
    conversation::{ChatMsg, Msg},
    user::User,
};

// TODO: look into druid's Arcstr because we can use that for some of these
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
    pub conversations: HashMap<String, Conversation>,
    pub selected_conv: Option<Conversation>,
    sub_id: String,
}

impl AppState {
    pub fn new() -> Self {
        let config = Config::load();
        let mut conversations = HashMap::new();
        for i in config.contacts.iter() {
            conversations.insert(i.pk.clone(), Conversation::new(i.clone()));
        }
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

    pub fn push_conv_msg(&mut self, msg: &Msg, conversation_pk: &str) {
        match self.conversations.get_mut(conversation_pk) {
            Some(conv) => conv.push_msg(msg),
            None => println!("Conversation not found!"),
        }
        if self.selected_conv.is_some() {
            self.selected_conv.as_mut().unwrap().push_msg(msg);
        }
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
