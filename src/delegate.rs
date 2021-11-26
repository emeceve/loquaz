use std::{
    str::FromStr,
    sync::{Arc, Mutex},
};

use druid::{AppDelegate, ExtEventSink, Handled, Selector};
use nostr::{util::nip04::decrypt, ClientMessage, Event};
use secp256k1::{
    schnorrsig::{self, PublicKey},
    SecretKey,
};
use serde_json::json;
use tokio_tungstenite::{tungstenite::Message, MaybeTlsStream, WebSocketStream};

use crate::{
    data::{app_state::AppState, conversation::ChatMsg},
    ws_service::connect,
};

pub const WS_RECEIVED_DATA: Selector<String> = Selector::new("nost_client.received_data");
pub const CONNECT: Selector<ExtEventSink> = Selector::new("nostr_client.connect");
pub const SEND_MSG: Selector<ChatMsg> = Selector::new("nostr_client.send_msg");
pub const SEND_WS_MSG: Selector<String> = Selector::new("nostr_client.send_ws_msg");
pub const DELETE_CONTACT: Selector<String> = Selector::new("nostr_client.delete_contact");
pub const START_CHAT: Selector<String> = Selector::new("nostr_client.start_chat");
pub const SELECT_CONV: Selector<String> = Selector::new("nostr_client.select_conv");
pub const CONNECTED_RELAY: Selector<&str> = Selector::new("nostr_client.connected_relay");
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
        if let Some(val) = cmd.get(WS_RECEIVED_DATA) {
            match nostr::RelayMessage::from_json(val) {
                Ok(msg) => match msg {
                    nostr::RelayMessage::Notice { message } => {
                        println!("{}", message);
                    }
                    nostr::RelayMessage::Event {
                        event,
                        subscription_id,
                    } => {
                        //TODO: Necessary make pub content field
                        match decrypt(
                            &SecretKey::from_str(&data.user.sk).unwrap(),
                            &event.pubkey,
                            &event.content,
                        ) {
                            Ok(decrypted_msg) => {
                                data.push_new_msg(ChatMsg::new("", &decrypted_msg))
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
            data.config.save();
            let (tx, rx) = futures_channel::mpsc::unbounded();
            data.tx = Arc::new(Mutex::new(Some(tx)));
            tokio::spawn(connect(
                ext_event_sink.clone(),
                data.config.ws_url.clone(),
                rx,
            ));
            Handled::Yes
        } else if let Some(msg) = cmd.get(SEND_MSG) {
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
            match &*data.tx.lock().unwrap() {
                None => {
                    println!("Null")
                }
                Some(tx) => tx.unbounded_send(Message::binary(msg.as_bytes())).unwrap(),
            }
            Handled::Yes
        } else if let Some(_) = cmd.get(CONNECTED_RELAY) {
            println!("Connected to relay");

            let authors = data.get_authors();
            let id = data.gen_sub_id();
            let req_sub = nostr::ClientMessage::new_req(
                id,
                nostr::SubscriptionFilter::new()
                    .authors(authors)
                    .kind(nostr::Kind::EncryptedDirectMessage).tag_p(data.user.keys.as_ref().unwrap().as_ref().public_key),
            );
            // let req_sub = json!(["REQ", id, { "authors": authors, "kind": 4 }]).to_string();
            println!("Sending subscription REQ");
            ctx.submit_command(SEND_WS_MSG.with(req_sub.to_json()));

            Handled::Yes
        } else if let Some(pk) = cmd.get(DELETE_CONTACT) {
            data.delete_contact(pk);
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
