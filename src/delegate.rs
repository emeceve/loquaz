use std::sync::{Arc, Mutex};

use druid::{AppDelegate, ExtEventSink, Handled, Selector};
use tokio::net::TcpStream;
use tokio_tungstenite::{tungstenite::Message, MaybeTlsStream, WebSocketStream};

use crate::{
    data::{ChatMsg, State, TxOrNull},
    ws_service::connect,
};

pub const WS_RECEIVED_DATA: Selector<String> = Selector::new("nost_client.received_data");
pub const CONNECT: Selector<ExtEventSink> = Selector::new("nostr_client.connect");
pub const SEND_MSG: Selector<ChatMsg> = Selector::new("nostr_client.send_msg");
pub const DELETE_CONTACT: Selector<String> = Selector::new("nostr_client.delete_contact");
pub const START_CHAT: Selector<String> = Selector::new("nostr_client.start_chat");
//pub const CONNECTED: Selector<Arc<WebSocketStream<MaybeTlsStream<TcpStream>>>> =
//    Selector::new("nostr-client.connected");

pub struct Delegate;

impl AppDelegate<State> for Delegate {
    fn command(
        &mut self,
        ctx: &mut druid::DelegateCtx,
        target: druid::Target,
        cmd: &druid::Command,
        data: &mut State,
        env: &druid::Env,
    ) -> druid::Handled {
        if let Some(val) = cmd.get(WS_RECEIVED_DATA) {
            data.push_new_msg(ChatMsg::new("", &val));
            Handled::Yes
        } else if let Some(ext_event_sink) = cmd.get(CONNECT) {
            let (tx, rx) = futures_channel::mpsc::unbounded();
            data.tx = Arc::new(Mutex::new(TxOrNull::Tx(tx)));
            tokio::spawn(connect(ext_event_sink.clone(), data.ws_url.clone(), rx));
            Handled::Yes
        } else if let Some(msg) = cmd.get(SEND_MSG) {
            match &*data.tx.lock().unwrap() {
                TxOrNull::Null => {
                    println!("Null")
                }
                TxOrNull::Tx(tx) => tx
                    .unbounded_send(Message::binary(msg.content.as_bytes()))
                    .unwrap(),
            }
            Handled::Yes
        } else if let Some(pk) = cmd.get(DELETE_CONTACT) {
            data.delete_contact(pk);
            Handled::Yes
        } else if let Some(pk) = cmd.get(START_CHAT) {
            data.set_current_chat(pk);
            Handled::Yes
        } else {
            Handled::No
        }
    }
}
