use druid::{
    im::{vector, Vector},
    Data, Lens,
};

use crate::core::conversations::{Conversation, Message, MessageSource};

use super::contact_state::ContactState;

#[derive(Clone, Data, Lens, Debug)]
pub struct ConversationState {
    pub contact: ContactState,
    pub messages: Vector<MessageState>,
    pub new_message: String,
}

impl ConversationState {
    pub fn new(contact: ContactState, messages: Vec<MessageState>) -> Self {
        Self {
            contact,
            messages: Vector::from(messages),
            new_message: "".into(),
        }
    }

    pub fn from_entity(conv: Conversation) -> Self {
        Self {
            contact: ContactState::new(&conv.contact.alias, &conv.contact.pk.to_string()),
            messages: conv
                .messages
                .into_iter()
                .map(|m| MessageState::from_entity(m))
                .collect(),
            new_message: "".into(),
        }
    }

    pub fn push_msg(&mut self, msg: &MessageState) {
        self.messages.push_front(msg.clone());
    }
}

#[derive(Clone, Data, Lens, Debug)]
pub struct MessageState {
    pub source: MessageSourceState,
    pub content: String,
}

impl MessageState {
    pub fn new(source: MessageSourceState, content: &str) -> Self {
        Self {
            source,
            content: content.into(),
        }
    }

    pub fn from_entity(message: Message) -> Self {
        Self {
            source: match message.source {
                MessageSource::Me => MessageSourceState::Me,
                MessageSource::Them => MessageSourceState::Them,
            },
            content: message.content,
        }
    }
}

#[derive(Clone, Data, PartialEq, Debug)]
pub enum MessageSourceState {
    Me,
    Them,
}

#[derive(Clone, Debug)]
pub struct NewMessage {
    pub pk: String,
    pub content: String,
}

impl NewMessage {
    pub fn new(pk: &str, content: &str) -> Self {
        Self {
            pk: pk.into(),
            content: content.into(),
        }
    }
}
