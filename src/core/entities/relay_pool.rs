use std::collections::HashMap;

use futures::{SinkExt, StreamExt};
use nostr::{self, SubscriptionFilter};
use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio_tungstenite::{connect_async, tungstenite::Message};

use super::subscription::{Channel, Subscription};

pub struct RelayPool {
    relays: HashMap<String, Relay>,
    pool_receiver: Receiver<RelayPoolEv>,
    pool_sender: Sender<RelayPoolEv>,
    subscription: Subscription,
}

impl RelayPool {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel(32);
        Self {
            relays: HashMap::new(),
            pool_receiver: receiver,
            pool_sender: sender,
            subscription: Subscription::new(),
        }
    }
    pub fn add(&mut self, relay_url: &str) {
        self.relays.insert(
            relay_url.clone().into(),
            Relay::new(relay_url, self.pool_sender.clone()),
        );
    }

    pub async fn start_sub(&mut self, filters: Vec<SubscriptionFilter>) {
        self.subscription.update_filters(filters.clone());
        for (k, v) in self.relays.iter() {
            match v.status {
                RelayStatus::Connected => {
                    let channel = Channel::new(&v.url);
                    v.send_msg(nostr::ClientMessage::new_req(
                        channel.id.clone(),
                        filters.clone(),
                    ))
                    .await;
                    self.subscription.add_channel(channel);
                }
                _ => (),
            }
        }
    }

    pub async fn connect_relay(&mut self, url: &str) {
        self.relays.get_mut(url.into()).unwrap().connect().await;
    }
    pub async fn disconnect_relay(&mut self, url: &str) {
        self.relays.get_mut(url.into()).unwrap().disconnect().await;
    }
}

#[derive(Debug)]
pub struct Relay {
    url: String,
    status: RelayStatus,
    pool_sender: Sender<RelayPoolEv>,
    relay_sender: Option<Sender<RelayEv>>,
}

impl Relay {
    pub fn new(url: &str, pool_sender: Sender<RelayPoolEv>) -> Self {
        Self {
            url: url.into(),
            status: RelayStatus::Disconnected,
            pool_sender,
            relay_sender: None,
        }
    }

    pub async fn connect(&mut self) {
        let url = url::Url::parse(&self.url).unwrap();
        println!("Trying to connect {} ...", url.to_string());
        let (ws_stream, _) = connect_async(&url).await.expect("Failed to connect!");
        println!("Successfully connected to relay {}!", &url.to_string());
        self.status = RelayStatus::Connected;

        let (mut ws_tx, mut ws_rx) = ws_stream.split();

        let (relay_sender, mut relay_receiver) = mpsc::channel::<RelayEv>(32);
        self.relay_sender = Some(relay_sender);
        let url_clone = url.clone().to_string();
        tokio::spawn(async move {
            while let Some(relay_ev) = relay_receiver.recv().await {
                match relay_ev {
                    RelayEv::SendMsg(msg) => {
                        println!("Sending message {}", msg.to_json());
                        ws_tx.send(Message::Text(msg.to_json())).await;
                    }
                    RelayEv::Close => {
                        ws_tx.close().await;
                        relay_receiver.close();
                    }
                }
            }
            println!("Closed RELAY TX to WS RX {}", url_clone);
        });

        let pool_sender = self.pool_sender.clone();
        tokio::spawn(async move {
            let relay_url = url.to_string();
            while let Some(msg_res) = ws_rx.next().await {
                match msg_res {
                    Ok(msg) => {
                        let data = msg.into_data();
                        let data_to_str = String::from_utf8(data).unwrap();
                        println!("Received data {}", &data_to_str);
                        match nostr::RelayMessage::from_json(&data_to_str) {
                            Ok(msg) => {
                                //                               println!("[Received RelayMsg]: {}", &msg.to_json());
                                match pool_sender
                                    .send(RelayPoolEv::ReceivedMsg {
                                        relay_url: relay_url.clone(),
                                        msg,
                                    })
                                    .await
                                {
                                    Ok(_) => println!("[CH Relay -> RelayPool] Sent to relay pool"),
                                    Err(err) => println!("[CH Relay -> RelayPool] {}", err),
                                };
                            }
                            Err(err) => println!("{}", err),
                        }
                    }
                    Err(err) => println!("{}", err),
                }
            }
            pool_sender.send(RelayPoolEv::RelayDisconnected {
                relay_url: relay_url.clone(),
            });
            println!("Closed WS RX to RELAY POOL TX {}", relay_url);
        });
    }

    pub async fn disconnect(&mut self) {
        self.send_relay_ev(RelayEv::Close).await;
        self.status = RelayStatus::Disconnected;
    }

    pub async fn send_msg(&self, msg: nostr::ClientMessage) {
        self.send_relay_ev(RelayEv::SendMsg(msg)).await;
    }

    async fn send_relay_ev(&self, relay_msg: RelayEv) {
        if self.relay_sender.is_some() {
            self.relay_sender.clone().unwrap().send(relay_msg).await;
        }
    }
}

#[derive(Debug)]
pub enum RelayStatus {
    Disconnected,
    Connected,
    Connecting,
}

#[derive(Debug)]
pub enum RelayPoolEv {
    RelayDisconnected {
        relay_url: String,
    },
    ReceivedMsg {
        relay_url: String,
        msg: nostr::RelayMessage,
    },
}

#[derive(Debug)]
enum RelayEv {
    SendMsg(nostr::ClientMessage),
    Close,
}
