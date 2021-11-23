use crate::{data::State, pages::chat::view::chat_tab, pages::config::view::config_tab};
use druid::{widget::Tabs, Widget};

pub fn root_ui() -> impl Widget<State> {
    Tabs::new()
        .with_tab("Chat", chat_tab())
        .with_tab("Config", config_tab())
}
