// Copyright 2019-2021 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod broker;
mod cmd;
mod core;

use broker::{start_broker, BrokerEvent};

use crate::cmd::{
    add_contact, add_relay, generate_key_pair, get_config, remove_contact, remove_relay,
    restore_key_pair,
};

use tokio::sync::mpsc;
pub struct AppState {
    pub core_command_sender: mpsc::Sender<BrokerEvent>,
}

use tauri::Manager;

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let main_window = app.get_window("main").unwrap();
            println!("Initing process");
            //           tauri::async_runtime::spawn(async move {
            //    start_broker(main_window);
            //  main_window.emit("test-event", "Test").unwrap();
            //            });
            let (sender, receiver) = mpsc::channel::<BrokerEvent>(64);

            tokio::spawn(start_broker(receiver));

            let app_handle = app.handle();
            app_handle.manage(AppState {
                core_command_sender: sender,
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_config,
            restore_key_pair,
            add_contact,
            add_relay,
            remove_relay,
            remove_contact,
            generate_key_pair
        ])
        .run(tauri::generate_context!("tauri.conf.json"))
        .expect("error while running tauri application");
}
