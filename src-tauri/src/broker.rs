use log::{debug, error, info};
use tauri::Wry;
use tokio::sync::{mpsc, oneshot};

use crate::core::{
    config::Contact,
    conversations::{Conversation, ConvsNotifications},
    core::{CoreTaskHandle, CoreTaskHandleEvent},
};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum BrokerEventError {
    #[error("Failed Send")]
    FailedSend,
    #[error("Command failed: `{0}`")]
    CommandFailed(String),
}

pub enum BrokerEvent {
    AddRelay {
        url: String,
        resp: Responder<Result<(), String>>,
    },
    RemoveRelay {
        url: String,
        resp: Responder<Result<(), String>>,
    },
    ConnectRelay {
        url: String,
    },
    DisconnectRelay {
        url: String,
    },
    AddContact {
        new_contact: Contact,
        resp: Responder<()>,
    },
    RemoveContact {
        contact: Contact,
        resp: Responder<()>,
    },
    SubscribeInRelays {
        pk: String,
    },
    RestoreKeyPair {
        sk: String,
        resp: Responder<Result<(String, String), String>>,
    },
    GenerateNewKeyPair {
        resp: Responder<Result<(String, String), String>>,
    },
    SetConversation {
        pk: String,
    },
    GetConversation {
        pk: String,
        resp: Responder<Result<Conversation, String>>,
    },
    SendMessage {
        pk: String,
        content: String,
    },
    LoadConfigs {
        resp: Responder<(Vec<String>, Vec<Contact>)>,
    },
}

pub type Responder<T> = oneshot::Sender<T>;

//pub enum BrokerNotification {
//    ConfigUpdated { config: ConfigState },
//}

async fn handle_broker_event(
    broker_event: BrokerEvent,
    core_handle: &mut CoreTaskHandle,
) -> Result<(), BrokerEventError> {
    match broker_event {
        BrokerEvent::SendMessage { pk, content } => {
            core_handle
                .send_msg_to_contact(&pk, &content)
                .await
                .map_err(|err| BrokerEventError::CommandFailed(err.to_string()))?;
            Ok(())
        }
        BrokerEvent::SetConversation { pk } => {
            if let Some(_conv) = core_handle.get_conv(pk) {
                //    event_sink.add_idle_callback(move |data: &mut AppState| {
                //        data.selected_conv = Some(ConversationState::from_entity(conv));
                //    });
            };

            Ok(())
        }
        BrokerEvent::GetConversation { pk, resp } => {
            if let Some(conv) = core_handle.get_conv(pk) {
                resp.send(Ok(conv))
                    .map_err(|_e| BrokerEventError::FailedSend)?
            };
            Ok(())
        }
        BrokerEvent::RestoreKeyPair { sk, resp } => {
            core_handle.import_user_sk(sk.clone());
            resp.send(Ok((sk, core_handle.get_user().get_pk().to_string())))
                .map_err(|_e| BrokerEventError::FailedSend)?;

            Ok(core_handle.subscribe().await)
        }
        BrokerEvent::GenerateNewKeyPair { resp } => {
            core_handle.gen_new_user_keypair();
            let user = core_handle.get_user();
            resp.send(Ok((
                user.get_sk().unwrap().to_string(),
                user.get_pk().to_string(),
            )))
            .map_err(|_e| BrokerEventError::FailedSend);
            core_handle.subscribe().await;
            Ok(())
        }

        BrokerEvent::AddRelay { url, resp } => {
            if let CoreTaskHandleEvent::RelayAdded(Ok(_)) = core_handle.add_relay(url) {
                resp.send(Ok(())).map_err(|_e| BrokerEventError::FailedSend)
                //   update_config_state(&event_sink, &core_handle).await;
            } else {
                resp.send(Err(format!("Could not add relay")))
                    .map_err(|_e| BrokerEventError::FailedSend)
            }
        }
        BrokerEvent::RemoveRelay { url, resp } => {
            if let CoreTaskHandleEvent::RemovedRelay(Ok(_)) = core_handle.remove_relay(url) {
                resp.send(Ok(())).map_err(|_e| BrokerEventError::FailedSend)
            } else {
                resp.send(Err(format!("Failed to remove")))
                    .map_err(|_e| BrokerEventError::FailedSend)
            }
        }
        BrokerEvent::ConnectRelay { url } => Ok(core_handle.connect_relay(url).await),
        BrokerEvent::DisconnectRelay { url } => Ok(core_handle.disconnect_relay(url).await),
        BrokerEvent::SubscribeInRelays { pk: _ } => Ok(core_handle.subscribe().await),
        BrokerEvent::AddContact { new_contact, resp } => {
            let _res = core_handle.add_contact(new_contact);
            resp.send(()).map_err(|_e| BrokerEventError::FailedSend);

            //Update filters and resubscribe based on updated conversations

            Ok(core_handle.subscribe().await)
        }
        BrokerEvent::RemoveContact { contact, resp } => {
            let _res = core_handle.remove_contact(contact).await;
            resp.send(()).map_err(|_e| BrokerEventError::FailedSend)
        }
        BrokerEvent::LoadConfigs { resp } => resp
            .send(core_handle.get_config())
            .map_err(|_e| BrokerEventError::FailedSend),
    }
}

pub async fn start_broker(
    mut broker_receiver: mpsc::Receiver<BrokerEvent>,
    main_window: tauri::Window<Wry>,
) {
    let mut core_handle = CoreTaskHandle::new();

    let mut rec_convs_noti = core_handle.get_convs_notifications();
    //  let ev_sink_clone = event_sink.clone();

    tokio::spawn(async move {
        while let Ok(noti) = rec_convs_noti.recv().await {
            match noti {
                ConvsNotifications::NewMessage(new_msg) => {
                    debug!("{:?}", new_msg);
                    main_window
                        .emit("new_message", new_msg)
                        .expect("Can't communicate back to the main window");
                }
            }
        }
    });

    core_handle.connect_all_relays().await;
    core_handle.subscribe().await;
    info!("Broker initialized and waiting for commands");
    while let Some(broker_event) = broker_receiver.recv().await {
        if let Err(e) = handle_broker_event(broker_event, &mut core_handle).await {
            error!("broker_event error: {:?}", e.to_string())
        }
    }
}
