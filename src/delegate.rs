use std::{rc::Rc, str::FromStr, sync::Arc};

use druid::{AppDelegate, ExtEventSink, Handled, Selector};
use nostr::{util::nip04::decrypt, ClientMessage, Event};
use secp256k1::SecretKey;

use crate::{
    broker::BrokerNotification,
    data::{
        app_state::AppState,
        state::{
            config_state::ConfigState,
            contact_state::ContactState,
            conversation_state::{ChatMsgState, MsgState},
        },
    },
};

pub const WS_RECEIVED_DATA: Selector<String> = Selector::new("nost_client.received_data");
pub const CONNECT: Selector<ExtEventSink> = Selector::new("nostr_client.connect");
pub const SEND_MSG: Selector<ChatMsgState> = Selector::new("nostr_client.send_msg");
pub const SEND_WS_MSG: Selector<String> = Selector::new("nostr_client.send_ws_msg");
pub const REMOVE_CONTACT: Selector<ContactState> = Selector::new("nostr_client.delete_contact");
pub const REMOVE_RELAY: Selector<String> = Selector::new("nostr_client.remove_relay");
pub const CONNECT_RELAY: Selector<String> = Selector::new("nostr_client.connect_relay");
pub const DISCONNECT_RELAY: Selector<String> = Selector::new("nostr_client.disconnect_relay");
pub const START_CHAT: Selector<String> = Selector::new("nostr_client.start_chat");
pub const SELECT_CONV: Selector<String> = Selector::new("nostr_client.select_conv");
pub const CONNECTED_RELAY: Selector<&str> = Selector::new("nostr_client.connected_relay");
pub const BROKER_NOTI: Selector<BrokerNotification> = Selector::new("nostr_client.broker_noti");
//pub const CONNECTED: Selector<Arc<WebSocketStream<MaybeTlsStream<TcpStream>>>> =
//    Selector::new("nostr-client.connected");

pub struct Delegate;

impl AppDelegate<AppState> for Delegate {
    fn command(
        &mut self,
        ctx: &mut druid::DelegateCtx,
        target: druid::Target,
        cmd: &druid::Command,
        data: &mut AppState,
        env: &druid::Env,
    ) -> druid::Handled {
        if let Some(note) = cmd.get(BROKER_NOTI) {
            match note {
                BrokerNotification::ConfigUpdated { config } => {
                    data.config = config.to_owned();
                }
            }
            Handled::Yes
        } else if let Some(val) = cmd.get(WS_RECEIVED_DATA) {
            match nostr::RelayMessage::from_json(val) {
                Ok(msg) => match msg {
                    nostr::RelayMessage::Notice { message } => {
                        println!("{}", message);
                    }
                    nostr::RelayMessage::Event {
                        event,
                        subscription_id: _,
                    } => {
                        match decrypt(
                            &SecretKey::from_str(&data.user.sk).unwrap(),
                            &event.pubkey,
                            &event.content,
                        ) {
                            Ok(decrypted_msg) => {
                                let user_pk = &data.user.keys.clone().unwrap().public_key;
                                let mut author: String = "".into();
                                let mut conversation_pk: String = "".into();
                                //If I am the author
                                if event.pubkey == *user_pk {
                                    event.tags.iter().for_each(|e| {
                                        if e.kind() == "p" {
                                            author = user_pk.to_string().clone();
                                            conversation_pk = e.content().clone().to_string();
                                        }
                                    })
                                } else {
                                    author = event.pubkey.clone().to_string();
                                    conversation_pk = event.pubkey.clone().to_string();
                                }

                                //Prevents the situations where "p" tag is missing
                                if !author.is_empty() {
                                    data.push_conv_msg(
                                        &MsgState::new(&author, &decrypted_msg),
                                        &conversation_pk,
                                    );
                                }
                            }
                            Err(error) => eprintln!("{}", error),
                        };
                    }
                    nostr::RelayMessage::Empty => println!("Empty message"),
                },

                Err(err) => println!("{}", err),
            }

            Handled::Yes
        } else if let Some(ext_event_sink) = cmd.get(CONNECT) {
            // data.config.save();

            //    ctx.submit_command(CONNECTED_RELAY.with(""));
            //            let sender = (*data.sender_broker).clone();
            //            let url = data.config.ws_url.clone();
            //            if let Some(tx) = sender {
            //                tokio::spawn(async move {
            //                    tx.send(crate::broker::BrokerEvent::AddRelay { url: url })
            //                        .await;
            //                });
            //            }

            //            tokio::spawn(data.relay.as_mut().unwrap().connect());
            //            let (tx, rx) = futures_channel::mpsc::unbounded();
            //            data.tx = Arc::new(Mutex::new(Some(tx)));
            //            tokio::spawn(connect(
            //                ext_event_sink.clone(),
            //                data.config.ws_url.clone(),
            //                rx,
            //            ));
            Handled::Yes
        } else if let Some(msg) = cmd.get(SEND_MSG) {
            dbg!(&msg);
            let dm = Event::new_encrypted_direct_msg(
                // TODO is there a better way to deal with an Arc than all these as_refs?
                data.user.keys.as_ref().unwrap().as_ref(),
                &nostr::Keys::new_pub_only(&msg.receiver_pk).unwrap(),
                &msg.content,
            );

            let ev = ClientMessage::new_event(dm.unwrap());

            ctx.submit_command(SEND_WS_MSG.with(ev.to_json()));
            Handled::Yes
        } else if let Some(msg) = cmd.get(SEND_WS_MSG) {
            println!("Message ws to be sent: {}", msg);
            //           match data.relay.as_ref() {
            //               None => {
            //                   println!("Null")
            //               }
            //               Some(relay) => {
            //                   let relay_clone = relay.clone();
            //                   relay_clone.send_msg(msg.clone());
            //               }
            //           }
            Handled::Yes
        } else if let Some(_) = cmd.get(CONNECTED_RELAY) {
            println!("Connected to relay");

            let authors = data.get_authors();
            let id = data.gen_sub_id();
            let req_sub = nostr::ClientMessage::new_req(
                id,
                vec![nostr::SubscriptionFilter::new()
                    .authors(authors)
                    .kind(nostr::Kind::EncryptedDirectMessage)
                    .tag_p(data.user.keys.as_ref().unwrap().as_ref().public_key)],
            );
            println!("Sending subscription REQ");
            ctx.submit_command(SEND_WS_MSG.with(req_sub.to_json()));

            Handled::Yes
        } else if let Some(contact_state) = cmd.get(REMOVE_CONTACT) {
            data.delete_contact(contact_state);
            Handled::Yes
        } else if let Some(url) = cmd.get(REMOVE_RELAY) {
            data.remove_relay(url);
            Handled::Yes
        } else if let Some(url) = cmd.get(CONNECT_RELAY) {
            data.connect_relay(url);
            Handled::Yes
        } else if let Some(url) = cmd.get(DISCONNECT_RELAY) {
            let sender = (*data.sender_broker).clone();
            let url_clone = String::from(url);
            tokio::spawn(async move {
                sender
                    .unwrap()
                    .send(crate::broker::BrokerEvent::DisconnectRelay { url: url_clone })
                    .await;
            });
            Handled::Yes
        } else if let Some(pk) = cmd.get(START_CHAT) {
            data.set_current_chat(pk);
            Handled::Yes
        } else if let Some(pk) = cmd.get(SELECT_CONV) {
            data.set_conv(pk);
            Handled::Yes
        } else {
            Handled::No
        }
    }
}
