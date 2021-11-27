use druid::{
    im::{vector, Vector},
    Data, Lens,
};

use super::contact::Contact;

#[derive(Clone, Data, Lens, Debug)]
pub struct Conversation {
    pub contact: Contact,
    pub messages: Vector<Msg>,
}

// let conversation = HashMap<pk, Conversation>

impl Conversation {
    pub fn new(contact: Contact) -> Self {
        Self {
            contact,
            messages: vector![],
        }
    }

    pub fn push_msg(&mut self, msg: &Msg) {
        self.messages.push_front(msg.clone());
    }
}

#[derive(Clone, Data, Lens, Debug)]
pub struct Msg {
    pub source_pk: String,
    pub content: String,
}

impl Msg {
    pub fn new(source_pk: &str, content: &str) -> Self {
        Self {
            source_pk: source_pk.into(),
            content: content.into(),
        }
    }
}

#[derive(Clone, Debug)]
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
