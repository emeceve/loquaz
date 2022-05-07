use std::collections::HashMap;

use futures::{SinkExt, StreamExt};
use log::{debug, error};
use nostr::{self, ClientMessage, Event, Keys, RelayMessage, SubscriptionFilter};
use tokio::sync::{
    broadcast,
    mpsc::{self, Receiver, Sender},
};
use tokio_tungstenite::{connect_async, tungstenite::Message};

use super::{config::Contact, subscription::Subscription};

pub struct RelayPoolTask {
    receiver: Receiver<RelayPoolEv>,
    notification_sender: broadcast::Sender<RelayPoolNotifications>,
    events: HashMap<String, Event>,
}

impl RelayPoolTask {
    pub fn new(
        pool_task_receiver: Receiver<RelayPoolEv>,
        notification_sender: broadcast::Sender<RelayPoolNotifications>,
    ) -> Self {
        Self {
            receiver: pool_task_receiver,
            events: HashMap::new(),
            notification_sender,
        }
    }

    async fn handle_message(&mut self, msg: RelayPoolEv) {
        match msg {
            RelayPoolEv::ReceivedMsg { relay_url, msg } => {
                // TODO: set up optional logging
                debug!("Received message from {}: {:?}", &relay_url, &msg);
                match msg {
                    RelayMessage::Event {
                        event,
                        subscription_id: _,
                    } => {
                        //Verifies if the event is valid
                        if let Ok(_) = event.verify() {
                            //Adds only new events
                            if let None = self.events.insert(event.id.to_string(), event.clone()) {
                                // TODO: set up optional logging
                                debug!("New event, propagates");
                                if let Err(e) = self
                                    .notification_sender
                                    .send(RelayPoolNotifications::ReceivedEvent { ev: event })
                                {
                                    error!("RelayPoolNotifications::ReceivedEvent error: {:?}", e);
                                };
                            }
                        }
                    }
                    _ => (),
                }
            }
            RelayPoolEv::EventSent { ev } => {
                self.events.insert(ev.id.to_string(), ev.clone());
            }
            RelayPoolEv::RemoveContactEvents(contact_keys) => {
                self.events.retain(|_, v| {
                    v.pubkey != contact_keys.public_key
                        && v.tags[0].content() != contact_keys.public_key.to_string()
                });
            }

            _ => (),
        }
    }
}

async fn start_relay_pool_task(mut task: RelayPoolTask) {
    while let Some(msg) = task.receiver.recv().await {
        task.handle_message(msg).await;
    }
}

pub struct RelayPool {
    relays: HashMap<String, Relay>,
    pool_task_sender: Sender<RelayPoolEv>,
    subscription: Subscription,
    notification_receiver: broadcast::Receiver<RelayPoolNotifications>,
    notification_sender: broadcast::Sender<RelayPoolNotifications>,
}

impl RelayPool {
    pub fn new() -> Self {
        let (notification_sender, notification_receiver) = broadcast::channel(64);
        let (sender, receiver) = mpsc::channel(64);
        let relay_pool_task = RelayPoolTask::new(receiver, notification_sender.clone());
        tokio::spawn(start_relay_pool_task(relay_pool_task));
        Self {
            relays: HashMap::new(),
            pool_task_sender: sender,
            subscription: Subscription::new(),
            notification_receiver,
            notification_sender,
        }
    }
    pub fn get_notifications_ch(&self) -> broadcast::Receiver<RelayPoolNotifications> {
        self.notification_sender.subscribe()
    }
    pub fn add(&mut self, relay_url: &str) {
        self.relays.insert(
            relay_url.clone().into(),
            Relay::new(relay_url, self.pool_task_sender.clone()),
        );
    }

    pub fn list_relays(&self) -> Vec<Relay> {
        self.relays.iter().map(|(_k, v)| v.to_owned()).collect()
    }
    pub async fn remove_contact_events(&self, contact: Contact) {
        //TODO: Remove this convertion when change contact pk to Keys type
        let c_keys = Keys::new_pub_only(&contact.pk.to_string()).unwrap();
        if let Err(e) = self
            .pool_task_sender
            .send(RelayPoolEv::RemoveContactEvents(c_keys))
            .await
        {
            error!("remove_contact_events send error: {}", e.to_string())
        };
    }
    pub async fn send_ev(&self, ev: Event) {
        //Send to pool task to save in all received events
        if let Err(e) = self
            .pool_task_sender
            .send(RelayPoolEv::EventSent { ev: ev.clone() })
            .await
        {
            error!("send_ev send error: {}", e.to_string())
        };
        let relays_clone = self.relays.clone();
        for (_k, v) in relays_clone.iter() {
            v.send_relay_ev(RelayEv::SendMsg(ClientMessage::new_event(ev.clone())))
                .await;
        }
    }

    pub async fn start_sub(&mut self, filters: Vec<SubscriptionFilter>) {
        self.subscription.update_filters(filters.clone());
        let relays_clone = self.relays.clone();
        for (k, _) in relays_clone.iter() {
            self.subscribe_relay(k).await;
        }
    }

    async fn subscribe_relay(&mut self, url: &str) {
        if let Some(relay) = self.relays.get(url) {
            match relay.status {
                RelayStatus::Connected => {
                    let channel = self.subscription.get_channel(url);
                    relay
                        .send_msg(nostr::ClientMessage::new_req(
                            channel.id.clone(),
                            self.subscription.get_filters(),
                        ))
                        .await;
                }
                _ => (),
            }
        }
    }
    async fn unsubscribe_relay(&mut self, url: &str) {
        if let Some(relay) = self.relays.get(url) {
            match relay.status {
                RelayStatus::Connected => {
                    if let Some(ch) = self.subscription.remove_channel(url) {
                        relay.send_msg(nostr::ClientMessage::close(ch.id)).await;
                    }
                }
                _ => (),
            }
        }
    }

    pub async fn connect_all(&mut self) {
        for (relay_url, _) in self.relays.clone().iter() {
            self.connect_relay(relay_url).await
        }
    }
    pub async fn connect_relay(&mut self, url: &str) {
        self.relays
            .get_mut::<String>(&String::from(url))
            .unwrap()
            .connect()
            .await;
        self.subscribe_relay(url).await;
    }
    pub async fn disconnect_relay(&mut self, url: &str) {
        self.unsubscribe_relay(url).await;
        self.relays
            .get_mut::<String>(&String::from(url))
            .unwrap()
            .disconnect()
            .await;
    }
}

#[derive(Debug, Clone)]
pub struct Relay {
    pub url: String,
    pub status: RelayStatus,
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
        debug!("Trying to connect {} ...", url.to_string());

        //TODO: Maybe propagate errors
        if let Ok((ws_stream, _)) = connect_async(&url).await {
            debug!("Successfully connected to relay {}!", &url.to_string());
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
                            if let Err(e) = ws_tx.send(Message::Text(msg.to_json())).await {
                                error!("RelayEv::SendMsg error: {:?}", e);
                            };
                        }
                        RelayEv::Close => {
                            if let Err(e) = ws_tx.close().await {
                                error!("RelayEv::Close error: {:?}", e);
                            };
                            relay_receiver.close();
                        }
                    }
                }
                debug!("Closed RELAY TX to WS RX {}", url_clone);
            });

            let pool_sender = self.pool_sender.clone();
            tokio::spawn(async move {
                let relay_url = url.to_string();
                while let Some(msg_res) = ws_rx.next().await {
                    match msg_res {
                        Ok(msg) => {
                            let data = msg.into_data();
                            let data_to_str = String::from_utf8(data).unwrap();
                            //   println!("Received data {}", &data_to_str);
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
                                        Ok(_) => {
                                            debug!("[CH Relay -> RelayPool] Sent to relay pool");
                                        }
                                        Err(err) => {
                                            debug!("[CH Relay -> RelayPool] {}", &err);
                                        }
                                    }
                                }
                                Err(err) => {
                                    error!("{}", err);
                                }
                            }
                        }
                        Err(err) => {
                            error!("{}", err);
                        }
                    }
                }
                if let Err(e) = pool_sender
                    .send(RelayPoolEv::RelayDisconnected {
                        relay_url: relay_url.clone(),
                    })
                    .await
                {
                    error!("pool_send error: {}", e.to_string())
                };
                error!("Closed WS RX to RELAY POOL TX {}", relay_url);
            });
        }
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
            if let Err(e) = self.relay_sender.clone().unwrap().send(relay_msg).await {
                error!("send_relay_ev error: {}", e.to_string())
            };
        }
    }
}

#[derive(Debug, Clone)]
pub enum RelayStatus {
    Disconnected,
    Connected,
    _Connecting,
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
    RemoveContactEvents(Keys),
    EventSent {
        ev: Event,
    },
}
#[derive(Debug, Clone)]
pub enum RelayPoolNotifications {
    ReceivedEvent { ev: Event },
    _RelaysStatusChanged { relays: Vec<Relay> },
}

#[derive(Debug)]
enum RelayEv {
    SendMsg(nostr::ClientMessage),
    Close,
}
