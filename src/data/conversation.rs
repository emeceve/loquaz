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
    pub source: String,
    pub content: String,
}

impl Msg {
    pub fn new(source: &str, content: &str) -> Self {
        Self {
            source: source.into(),
            content: content.into(),
        }
    }
}
