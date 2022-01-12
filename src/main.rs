mod delegate;
mod view;

mod broker;
mod components;
mod core;
mod data;
mod menus;
mod pages;
mod theme;

use std::sync::Arc;

use broker::start_broker;
use data::app_state::AppState;
use delegate::Delegate;
use druid::{AppLauncher, WidgetExt, WindowDesc};
use menus::ContextMenuController;
use view::root_ui;

#[tokio::main]
async fn main() {
    let main_window = WindowDesc::new(root_ui().controller(ContextMenuController))
        .title("Nostr Chat")
        // .menu(menus::make_menu)
        .menu(menus::make_menu)
        .window_size((640., 580.))
        .with_min_size((600., 600.));
    let laucher = AppLauncher::with_window(main_window).delegate(Delegate {});

    //Init state
    let mut init_state = AppState::new();
    //Channel sender from druid app to broker
    let (sender, mut receiver) = tokio::sync::mpsc::channel(32);
    init_state.sender_broker = Arc::new(Some(sender));

    //Spawn broker
    tokio::spawn(start_broker(laucher.get_external_handle(), receiver));

    laucher
        .configure_env(theme::set_env())
        .launch(init_state)
        .expect("Failed to start");
}
