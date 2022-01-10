use druid::{AppDelegate, Handled, Selector};

use crate::{
    broker::BrokerNotification,
    data::{
        app_state::AppState,
        state::{contact_state::ContactState, conversation_state::NewMessage},
    },
};

pub const SEND_MSG: Selector<NewMessage> = Selector::new("nostr_client.send_msg");
pub const REMOVE_CONTACT: Selector<ContactState> = Selector::new("nostr_client.delete_contact");
pub const REMOVE_RELAY: Selector<String> = Selector::new("nostr_client.remove_relay");
pub const CONNECT_RELAY: Selector<String> = Selector::new("nostr_client.connect_relay");
pub const DISCONNECT_RELAY: Selector<String> = Selector::new("nostr_client.disconnect_relay");
pub const START_CHAT: Selector<String> = Selector::new("nostr_client.start_chat");
pub const SELECT_CONV: Selector<String> = Selector::new("nostr_client.select_conv");
pub const BROKER_NOTI: Selector<BrokerNotification> = Selector::new("nostr_client.broker_noti");

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
        } else if let Some(msg) = cmd.get(SEND_MSG) {
            dbg!(&msg);
            let sender = (*data.sender_broker).clone();
            let pk = msg.pk.clone();
            let content = msg.content.clone();
            tokio::spawn(async move {
                sender
                    .unwrap()
                    .send(crate::broker::BrokerEvent::SendMessage { pk, content })
                    .await;
            });
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
        } else if let Some(pk) = cmd.get(SELECT_CONV) {
            data.set_conv(pk);
            Handled::Yes
        } else {
            Handled::No
        }
    }
}
