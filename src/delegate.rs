use std::sync::{Arc, Mutex};

use druid::{AppDelegate, ExtEventSink, Handled, Selector};
use serde_json::json;
use tokio_tungstenite::{tungstenite::Message, MaybeTlsStream, WebSocketStream};

use crate::{
    data::app_state::{AppState, ChatMsg},
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
            //Todo change ChatMsg struct to Msg
            data.push_new_msg(ChatMsg::new("", &val));
            //        handle_msg(&val);
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
            //TODO convert message and pk to nostr NIP4
            ctx.submit_command(SEND_WS_MSG.with(msg.content.clone()));
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
            let req_sub = json!(["REQ", id, { "authors": authors, "kind": 4 }]).to_string();
            println!("Sending subscription REQ");
            ctx.submit_command(SEND_WS_MSG.with(req_sub.to_string()));

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
