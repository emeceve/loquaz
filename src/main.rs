mod delegate;
mod view;
mod ws_service;

mod broker;
mod core;
mod data;
mod pages;

use std::sync::Arc;

use broker::start_broker;
use data::{app_state::AppState, state::config_state::ConfigState};
use delegate::Delegate;
use druid::{AppLauncher, WindowDesc};
use view::root_ui;

#[tokio::main]
async fn main() {
    let main_window = WindowDesc::new(root_ui()).title("Nostr Chat");
    let laucher = AppLauncher::with_window(main_window).delegate(Delegate {});

    //Init state
    let mut init_state = AppState::new();
    //Channel sender from druid app to broker
    let (sender, mut receiver) = tokio::sync::mpsc::channel(32);
    init_state.sender_broker = Arc::new(Some(sender));

    //Spawn broker
    tokio::spawn(start_broker(laucher.get_external_handle(), receiver));

    //Launch druid app
    laucher.launch(init_state).expect("Failed to start");
}
