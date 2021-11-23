use nostr::{Event, Message};
use secp256k1::{schnorrsig::PublicKey, SecretKey};
use serde_json::json;
use std::str::FromStr;

pub fn create_encrypted_direct_msg(sender_sk: &str, receiver_pk: &str, content: &str) -> String {
    let sk = SecretKey::from_str(&sender_sk).unwrap();
    let pk = PublicKey::from_str(receiver_pk).unwrap();
    let ev = Event::new_encrypted_direct_msg(sk, &pk, &content);
    create_event_msg(&ev)
}

fn create_event_msg(ev: &Event) -> String {
    json!(["EVENT", ev]).to_string()
}
pub fn create_connect_msg(pk_list: Vec<String>) -> String {
    let con_msg = json!(["REQ", "randomID", { "authors": pk_list }]);
    con_msg.to_string()
}

pub fn handle_msg(msg: &str) {
    let handled_msg = Message::handle(msg).expect("Failed to handle message");

    match handled_msg {
        Message::Empty => {
            println!("Empty msg");
        }
        Message::Ping => {
            println!("Got ping");
        }
        Message::Notice(notice) => {
            println!("Got notice: {}", notice);
        }
        Message::Event(ev) => {
            println!("{:?}", ev);
        }
    }
}
