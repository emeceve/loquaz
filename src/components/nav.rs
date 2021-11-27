use druid::{widget::Flex, Color, Widget, WidgetExt};

use crate::data::{app_state::AppState, router::Route};

use super::icons::{chat_icon, contact_icon, settings_icon};

pub fn nav() -> impl Widget<AppState> {
    let chat_button =
        chat_icon().on_click(|_, data: &mut AppState, _| data.route.goto(Route::Chat));
    let contacts_button =
        contact_icon().on_click(|_, data: &mut AppState, _| data.route.goto(Route::Contacts));
    let settings_button =
        settings_icon().on_click(|_, data: &mut AppState, _| data.route.goto(Route::Settings));

    return Flex::column()
        .must_fill_main_axis(true)
        .with_spacer(20.)
        .with_child(chat_button)
        .with_spacer(20.)
        .with_child(contacts_button)
        .with_spacer(20.)
        .with_child(settings_button)
        .with_flex_spacer(1.)
        .padding(20.)
        .background(Color::BLACK);
}
