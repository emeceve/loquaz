use crate::core::entities::relay_pool::{Relay, RelayPool};
use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio_tungstenite::tungstenite::Message;
pub struct RelayPoolProvider {
    state: RelayPool,
    pool_receiver: Receiver<Message>,
    pool_sender: Sender<Message>,
}

impl RelayPoolProvider {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel(32);
        Self {
            state: RelayPool::new(),
            pool_receiver: receiver,
            pool_sender: sender,
        }
    }

    pub fn add(&mut self, relay_url: &str) {
        self.state.add(relay_url);
    }
}
