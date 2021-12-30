use druid::{ExtEventSink, Target};
use tokio::sync::mpsc;

use crate::{
    core::{
        core::{CoreTaskHandle, CoreTaskHandleEvent},
        entities::contact::Contact,
    },
    data::{app_state::AppState, state::config_state::ConfigState},
    delegate::CORE_EV,
};

pub enum BrokerEvent {
    AddRelay { url: String },
    RemoveRelay { url: String },
    ConnectRelay { url: String },
    DisconnectRelay { url: String },
    AddContact { new_contact: Contact },
    RemoveContact { contact: Contact },
    SubscribeInRelays { pk: String },
    LoadConfigs,
}

pub async fn start_broker(
    event_sink: ExtEventSink,
    mut broker_receiver: mpsc::Receiver<BrokerEvent>,
    core_handle: CoreTaskHandle,
) {
    //Load configs
    // send_res_ev_to_druid(&event_sink, core_handle.load_configs().await);

    while let Some(broker_event) = broker_receiver.recv().await {
        match broker_event {
            BrokerEvent::AddRelay { url } => {
                if let CoreTaskHandleEvent::RelayAdded(Ok(_)) = core_handle.add_relay(url).await {
                    update_config_state(&event_sink, &core_handle).await;
                }
            }
            BrokerEvent::RemoveRelay { url } => {
                if let CoreTaskHandleEvent::RemovedRelay(Ok(_)) =
                    core_handle.remove_relay(url).await
                {
                    update_config_state(&event_sink, &core_handle).await;
                }
            }
            BrokerEvent::ConnectRelay { url } => {
                core_handle.connect_relay(url).await;
            }
            BrokerEvent::DisconnectRelay { url } => {
                core_handle.disconnect_relay(url).await;
            }

            BrokerEvent::SubscribeInRelays { pk } => {
                core_handle.subscribe(pk).await;
            }
            BrokerEvent::AddContact { new_contact } => {
                let res = core_handle.add_contact(new_contact).await;
                update_config_state(&event_sink, &core_handle).await;
            }
            BrokerEvent::RemoveContact { contact } => {
                let res = core_handle.remove_contact(contact).await;
                update_config_state(&event_sink, &core_handle).await;
            }
            BrokerEvent::LoadConfigs => {
                update_config_state(&event_sink, &core_handle).await;
            }
        }
    }
}

async fn update_config_state(event_sink: &ExtEventSink, core_handle: &CoreTaskHandle) {
    if let CoreTaskHandleEvent::ConfigLoaded(Ok(conf)) = core_handle.load_configs().await {
        event_sink.add_idle_callback(move |data: &mut AppState| {
            data.config = ConfigState::from_entity(&conf);
        });
    }
}

fn send_res_ev_to_druid(event_sink: &ExtEventSink, res: CoreTaskHandleEvent) {
    event_sink
        .submit_command(CORE_EV, res, Target::Auto)
        .expect("Error while send core events to Druid event sink!");
}
