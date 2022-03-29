use std::{collections::HashMap, str::FromStr};

use super::{config::Contact, user::User};
use chrono::Duration;
use nostr::{util::nip04::decrypt, Event};
use secp256k1::schnorrsig::PublicKey;
use thiserror::Error;
use tokio::sync::broadcast;

#[derive(Debug, Error)]
pub enum ConversationsError {
    #[error("Adding new message failed")]
    AddMessageFailed,
}
#[derive(Clone)]
pub enum ConvsNotifications {
    NewMessage(Message),
}

pub struct Conversations {
    convs: HashMap<String, Conversation>,
    conv_noti_sender: broadcast::Sender<ConvsNotifications>,
    conv_noti_receiver: broadcast::Receiver<ConvsNotifications>,
}

impl Conversations {
    pub fn new() -> Self {
        let (sender, receiver) = broadcast::channel(64);
        Self {
            convs: HashMap::new(),
            conv_noti_sender: sender,
            conv_noti_receiver: receiver,
        }
    }

    pub fn get_notifications_ch(&self) -> broadcast::Receiver<ConvsNotifications> {
        self.conv_noti_sender.subscribe()
    }

    pub fn try_add_message_from_ev(
        &mut self,
        ev: Event,
        user: &User,
    ) -> Result<(), ConversationsError> {
        let peer_pk;
        let source;
        //If the user is event's author, is necessary get peer PK from p tag
        //to decrypt
        if ev.pubkey == user.get_pk() {
            // TODO
            // Currently this check is unecessary since
            // will not be possible to receive other kind of events
            //
            //      if ev.tags.len() >= 1 {
            //          if ev.tags[0].kind() == "p" {
            //              peer_pk = PublicKey::from_str(ev.tags[0].content()).unwrap();
            //          }
            //      }
            //

            peer_pk = PublicKey::from_str(ev.tags[0].content()).unwrap();
            source = MessageSource::Me;
        } else {
            peer_pk = ev.pubkey;
            source = MessageSource::Them;
        }
        if let Some(conv) = self.get_mut_conv(&peer_pk.to_string()) {
            if let Some(sk) = user.get_sk() {
                match decrypt(&sk, &peer_pk, &ev.content) {
                    Ok(decrypted_msg) => {
                        let new_msg = Message::new(source, &decrypted_msg, ev);

                        conv.add_message(new_msg.clone());

                        //Send notification to listeners
                        self.conv_noti_sender
                            .send(ConvsNotifications::NewMessage(new_msg));
                        Ok(())
                    }
                    Err(e) => {
                        eprintln!("{}", e);
                        Err(ConversationsError::AddMessageFailed)
                    }
                }
            } else {
                Err(ConversationsError::AddMessageFailed)
            }
        } else {
            Err(ConversationsError::AddMessageFailed)
        }
    }

    pub fn add_conv(&mut self, conv: Conversation) {
        let pk = conv.contact.pk.to_string();
        self.convs.insert(pk, conv);
    }

    pub fn remove_conv(&mut self, pk: &str) {
        self.convs.remove(pk);
    }

    pub fn get_conv(&self, pk: &str) -> Option<&Conversation> {
        self.convs.get::<String>(&String::from(pk))
    }

    pub fn get_mut_conv(&mut self, pk: &str) -> Option<&mut Conversation> {
        self.convs.get_mut::<String>(&String::from(pk))
    }

    pub fn list_convs(&self) -> Vec<Conversation> {
        self.convs.iter().map(|(k, v)| v.to_owned()).collect()
    }
}

#[derive(Clone, Debug)]
pub struct Conversation {
    pub contact: Contact,
    pub messages: Vec<Message>,
}

impl Conversation {
    pub fn new(contact: Contact) -> Self {
        Self {
            contact,
            messages: vec![],
        }
    }

    fn add_message(&mut self, message: Message) {
        // If the message is from Them, and is less than one minute old, show an OS notification
        let current_time = chrono::offset::Utc::now();

        if message.ev.created_at > current_time - Duration::seconds(60) {
            if message.source == MessageSource::Them {
                //         if Notification::new()
                //             .summary(&self.contact.alias)
                //             .body(&message.content)
                //             .show()
                //             .is_err()
                //         {
                //             eprintln!("Couldn't show OS notification")
                //         }
            }
        }

        self.messages.push(message);
        self.messages
            .sort_by(|a, b| a.ev.created_at.cmp(&b.ev.created_at));
    }
}

#[derive(Clone, Debug)]
pub struct Message {
    pub source: MessageSource,
    pub content: String,
    ev: Event,
}

impl Message {
    pub fn new(source: MessageSource, content: &str, ev: Event) -> Self {
        Self {
            source,
            content: content.into(),
            ev,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MessageSource {
    Me,
    Them,
}
