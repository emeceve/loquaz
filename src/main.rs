mod data;
mod delegate;
mod nostr_service;
mod view;
mod ws_service;
use std::sync::{Arc, Mutex};

mod pages;

use data::{State, TxOrNull};
use delegate::Delegate;
use druid::{AppLauncher, WindowDesc};
use view::root_ui;

#[tokio::main]
async fn main() {
    let main_window = WindowDesc::new(root_ui()).title("Nostr Chat");
    let laucher = AppLauncher::with_window(main_window).delegate(Delegate {});

    let init_state = State::new();

    laucher.launch(init_state).expect("Failed to start");
}
