use std::str::FromStr;

use crate::{broker::BrokerEvent, core::config::Contact, AppState};
use log::debug;
use secp256k1::schnorrsig::PublicKey;
use tauri::command;
use tokio::sync::oneshot;

#[command]
pub async fn get_config(
    state: tauri::State<'_, AppState>,
) -> Result<(Vec<String>, Vec<Contact>), String> {
    debug!("get_config command called");
    let (res_tx, res_rx) = oneshot::channel();
    state
        .core_command_sender
        .send(BrokerEvent::LoadConfigs { resp: res_tx })
        .await;

    res_rx.await.map_err(|err| format!("{}", err))
}

#[command]
pub async fn restore_key_pair(
    sk: String,
    state: tauri::State<'_, AppState>,
) -> Result<String, String> {
    debug!("restore_key_pair command called");
    let (res_tx, res_rx) = oneshot::channel();
    state
        .core_command_sender
        .send(BrokerEvent::RestoreKeyPair { sk, resp: res_tx })
        .await;

    res_rx.await.map_err(|err| format!("{}", err)).unwrap()
}
#[command]
pub async fn generate_key_pair(
    state: tauri::State<'_, AppState>,
) -> Result<(String, String), String> {
    debug!("generate_key_pair command called");
    let (res_tx, res_rx) = oneshot::channel();
    state
        .core_command_sender
        .send(BrokerEvent::GenerateNewKeyPair { resp: res_tx })
        .await;

    res_rx.await.map_err(|err| format!("{}", err)).unwrap()
}

#[command]
pub async fn add_contact(
    alias: String,
    pk: String,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    debug!("add_contact command called");
    if let Ok(pk) = PublicKey::from_str(&pk) {
        let new_contact = Contact::new(&alias, pk);

        let (res_tx, res_rx) = oneshot::channel();
        state
            .core_command_sender
            .send(BrokerEvent::AddContact {
                new_contact,
                resp: res_tx,
            })
            .await;
        return res_rx.await.map_err(|err| format!("{}", err));
    }
    Err(format!("Invalid PK {}", pk))
}

#[command]
pub async fn remove_contact(
    contact: Contact,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    debug!("remove_contact command called");
    let (res_tx, res_rx) = oneshot::channel();

    state
        .core_command_sender
        .send(BrokerEvent::RemoveContact {
            contact,
            resp: res_tx,
        })
        .await;
    res_rx.await.map_err(|e| format!("{}", e))
}

#[command]
pub async fn add_relay(url: String, state: tauri::State<'_, AppState>) -> Result<(), String> {
    debug!("add_relay command called");
    let (res_tx, res_rx) = oneshot::channel();
    state
        .core_command_sender
        .send(BrokerEvent::AddRelay { url, resp: res_tx })
        .await;
    res_rx.await.map_err(|err| format!("{}", err)).unwrap()
}

#[command]
pub async fn remove_relay(url: String, state: tauri::State<'_, AppState>) -> Result<(), String> {
    debug!("remove_relay command called");
    let (res_tx, res_rx) = oneshot::channel();
    state
        .core_command_sender
        .send(BrokerEvent::RemoveRelay { url, resp: res_tx })
        .await;
    res_rx.await.map_err(|err| format!("{}", err)).unwrap()
}

#[command]
pub fn message(value: String, state: tauri::State<'_, AppState>) -> String {
    debug!("Received message from frontend {}", value);
    format!("Got message {} sdfsd", value)
}

//#[command]
//pub fn init_process(window: Window, state: tauri::State<'_, AppState>) {
//    println!("Initing process");
//    std::thread::spawn(move || loop {
//        window.emit("test-event", "Test").unwrap();
//        std::thread::sleep(std::time::Duration::from_secs(3));
//    });
//}
