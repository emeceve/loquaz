use std::sync::{Arc, Mutex};

use druid::im::HashMap;
use futures::{SinkExt, StreamExt};
use nostr::SubscriptionFilter;
use tokio::sync::mpsc;
use tokio_tungstenite::{connect_async, tungstenite::Message};
use url::Url;

pub struct RelayTaskHandle {
    tx: mpsc::Sender<RelayTaskCmd>,
}

impl RelayTaskHandle {
    pub fn new(url: &str) -> Self {
        let (tx, rx) = mpsc::channel(32);
        let relay_task = RelayTask::new(url, rx);
        tokio::spawn(connect_relay(relay_task));
        Self { tx }
    }

    pub fn send_msg(&self, msg: String) {
        //   self.tx
        //       .send(RelayTaskCmd::SendMsg {
        //           msg: Message::binary(msg.as_bytes()),
        //       })
        let tx = self.tx.clone();
        tokio::spawn(async move {
            tx.send(RelayTaskCmd::SendMsg {
                msg: Message::binary(msg.as_bytes()),
            })
            .await;
        });
    }

    pub fn disconnect(&self) {
        self.tx.send(RelayTaskCmd::Disconnect);
    }
}

#[derive(PartialEq)]
enum RelayStatus {
    Connected,
    Disconnected,
}

enum RelayTaskCmd {
    SendMsg { msg: Message },
    Disconnect,
}

struct RelayTask {
    pub url: Url,
    pub status: RelayStatus,
    rx: mpsc::Receiver<RelayTaskCmd>,
}

impl RelayTask {
    pub fn new(url: &str, rx: mpsc::Receiver<RelayTaskCmd>) -> Self {
        Self {
            status: RelayStatus::Disconnected,
            url: Url::parse(url).unwrap(),
            rx,
        }
    }
}

async fn connect_relay(mut relay_task: RelayTask) {
    println!("Trying to connect {} ...", &relay_task.url.to_string());
    let (mut ws_stream, _) = connect_async(&relay_task.url)
        .await
        .expect("Failed to connect!");
    println!(
        "Successfully connected to relay {}!",
        &relay_task.url.to_string()
    );

    relay_task.status = RelayStatus::Connected;

    //   let (mut ws_tx, mut ws_rx) = ws_stream.split();
    //    let (tx, mut rx) = mpsc::channel(32);
    //    self.tx = Some(tx);

    //    let from_msg_to_ws = rx.map(Ok).forward(ws_tx);
    //    tokio::spawn(from_msg_to_ws);

    let mut is_connected = true;
    loop {
        if !is_connected {
            break;
        }

        tokio::select! {
                Some(task_cmd) = relay_task.rx.recv() =>  {
                    match task_cmd {
                        RelayTaskCmd::SendMsg {msg} => {
                            ws_stream.send(msg).await.unwrap();
                        },
                        RelayTaskCmd::Disconnect => {
                            is_connected = false;
                            ws_stream.close(None).await;
                        }                         }
                },

                Some(rec_data) = ws_stream.next() =>  {
                  match rec_data {
                Ok(Message::Text(data)) => {
                    println!("Received string data {}", data);
                }
                Ok(Message::Binary(_)) => println!("Invalid Message format"),
                Ok(Message::Ping(_) | Message::Pong(_)) => println!("Ping pong ping pong ..."),
                Ok(Message::Close(_)) => {
                    println!("Disconnected from server");
                    ws_stream.close(None).await;
                    break;
                }
                Err(err) => {
                    println!("Error: {}", err.to_string());
                    ws_stream.close(None).await;
                    break;
                }
            }
          }
        }
    }
    //Will worth nothing since this
    //instance will be removed from earth
    //relayTask.disconnect();
}

pub struct Subscription {
    pub id: String,
    pub filter: SubscriptionFilter,
}

impl Subscription {
    pub fn close(&self) {}
}
