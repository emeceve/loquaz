use druid::{
    im::{vector, Vector},
    Data, Lens,
};

use super::contact_state::ContactState;

#[derive(Clone, Data, Lens, Debug)]
pub struct ConversationState {
    pub contact: ContactState,
    pub messages: Vector<MsgState>,
}

// let conversation = HashMap<pk, Conversation>

impl ConversationState {
    pub fn new(contact: ContactState) -> Self {
        Self {
            contact,
            messages: vector![],
        }
    }

    pub fn push_msg(&mut self, msg: &MsgState) {
        self.messages.push_front(msg.clone());
    }
}

#[derive(Clone, Data, Lens, Debug)]
pub struct MsgState {
    pub source_pk: String,
    pub content: String,
}

impl MsgState {
    pub fn new(source_pk: &str, content: &str) -> Self {
        Self {
            source_pk: source_pk.into(),
            content: content.into(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ChatMsgState {
    pub receiver_pk: String,
    pub content: String,
}

impl ChatMsgState {
    pub fn new(receiver_pk: &str, content: &str) -> Self {
        Self {
            receiver_pk: receiver_pk.into(),
            content: content.into(),
        }
    }
}
