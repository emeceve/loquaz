use crate::{
    components::nav::nav,
    data::{app_state::AppState, router::Route},
    pages::config::view::config_tab,
    pages::{chat::view::chat_tab, contacts::view::contacts_tab},
};
use druid::{
    widget::{Flex, ViewSwitcher},
    Widget, WidgetExt,
};

pub fn root_ui() -> impl Widget<AppState> {
    // Tabs::new()
    // .with_tab("Chat", chat_tab())
    // .with_tab("Config", config_tab())
    Flex::row().with_child(nav()).with_flex_child(
        ViewSwitcher::new(
            |data: &AppState, _| data.route,
            |route: &Route, _, _| match route {
                Route::Chat => chat_tab().boxed(),
                Route::Contacts => contacts_tab().boxed(),
                Route::Settings => config_tab().boxed(),
            },
        ),
        1.0,
    )
}
