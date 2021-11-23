use druid::{
    im::{vector, Vector},
    Application, Data, Env, EventCtx, Lens,
};
use std::{
    fs::File,
    io::{BufReader, Error},
    rc::Rc,
    str::FromStr,
    sync::{Arc, Mutex},
};

use serde::{Deserialize, Serialize};

use crate::delegate::{CONNECT, DELETE_CONTACT, SELECT_CONV, SEND_MSG, START_CHAT};
use tokio_tungstenite::tungstenite::Message;

use secp256k1::{rand::rngs::OsRng, schnorrsig, Secp256k1, SecretKey};

use crate::ws_service::connect;

pub enum TxOrNull {
    Tx(futures_channel::mpsc::UnboundedSender<Message>),
    Null,
}

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
pub struct State {
    pub secret_key: String,
    pub public_key: String,
    pub chat_messages: Vector<String>,
    pub msg_to_send: String,
    pub ws_url: String,
    pub tx: Arc<Mutex<TxOrNull>>,
    pub contacts: Vector<Contact>,
    pub new_contact_alias: String,
    pub new_contact_pk: String,
    pub current_chat_contact: Contact,
    pub conversations: Vector<Rc<Conversation>>,
    pub selected_conv: Option<Rc<Conversation>>,
}

impl State {
    pub fn new() -> Self {
        let contacts = Self::load_contacts_from_json();
        let conversations = contacts
            .clone()
            .iter_mut()
            .map(|contact| Rc::new(Conversation::new(contact.clone())))
            .collect();

        State {
            public_key: "".into(),
            secret_key: "".into(),
            chat_messages: vector!(),
            msg_to_send: "".into(),
            ws_url: "".into(),
            tx: Arc::new(Mutex::new(TxOrNull::Null)),
            contacts,
            new_contact_pk: "".into(),
            new_contact_alias: "".into(),
            current_chat_contact: Contact::new("", ""),
            conversations,
            selected_conv: None,
        }
    }

    pub fn push_new_msg(&mut self, new_msg: ChatMsg) {
        self.chat_messages.push_front(new_msg.content);
    }

    pub fn generate_sk(&mut self) {
        let secp = Secp256k1::new();
        let mut rng = OsRng::new().unwrap();

        if let Ok(sk) = SecretKey::from_str(&self.secret_key) {
            let keypair = schnorrsig::KeyPair::from_secret_key(&secp, sk);
            let pk = schnorrsig::PublicKey::from_keypair(&secp, &keypair);
            self.public_key = pk.to_string();
        } else {
            let (sk, pk) = secp.generate_keypair(&mut rng);
            self.secret_key = sk.to_string();
            self.public_key = pk.to_string()[2..].into();
        };
    }
    pub fn set_current_chat(&mut self, pk: &str) {
        for contact in self.contacts.iter() {
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
        self.contacts
            .push_back(Contact::new(&self.new_contact_alias, &self.new_contact_pk));
        self.save_contacts_to_json().unwrap();
        self.new_contact_alias = "".into();
        self.new_contact_pk = "".into();
    }

    pub fn delete_contact(&mut self, pk: &str) {
        self.contacts.retain(|contact| contact.pk != pk);
        self.save_contacts_to_json().unwrap();
    }

    fn load_contacts_from_json() -> Vector<Contact> {
        let file = File::open("contacts.json");

        match file {
            Ok(file) => {
                let reader = BufReader::new(file);
                let contacts = serde_json::from_reader(reader).unwrap_or(vec![]);
                Vector::from(contacts)
            }
            Err(_) => vector![],
        }
    }

    fn save_contacts_to_json(&self) -> Result<(), Error> {
        let contacts_vec: Vec<Contact> = self
            .contacts
            .iter()
            .map(|contac| contac.to_owned())
            .collect();
        let serialized = serde_json::to_string_pretty(&contacts_vec)?;
        std::fs::write("contacts.json", serialized)?;
        Ok(())
    }
}

#[derive(Debug, Data, Clone, Lens, Serialize, Deserialize)]
pub struct Contact {
    pub alias: String,
    pub pk: String,
}

impl Contact {
    pub fn new(alias: &str, pk: &str) -> Self {
        Self {
            alias: alias.into(),
            pk: pk.into(),
        }
    }
}

#[derive(Clone, Data, Lens, Debug)]
pub struct Conversation {
    contact: Contact,
    messages: Vector<Msg>,
}

impl Conversation {
    pub fn new(contact: Contact) -> Self {
        Self {
            contact,
            messages: vector![],
        }
    }

    pub fn click_select_conv(ctx: &mut EventCtx, data: &mut Self, _env: &Env) {
        ctx.submit_command(SELECT_CONV.with(data.contact.pk.clone()));
    }
}

#[derive(Clone, Data, Lens, Debug)]
pub struct Msg {
    source_pk: String,
    content: String,
}

impl Msg {
    pub fn new(source_pk: &str, content: &str) -> Self {
        Self {
            source_pk: source_pk.into(),
            content: content.into(),
        }
    }
}
