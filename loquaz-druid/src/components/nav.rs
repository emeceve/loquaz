use druid::{widget::Flex, Color, Widget, WidgetExt};

use crate::data::{app_state::AppState, router::Route};

use super::icons::{chat_icon, contact_icon, settings_icon};

pub fn nav() -> impl Widget<AppState> {
    // Using "disabled" to represent the active state
    let chat_button = chat_icon()
        .on_click(|_, data, _| data.route.goto(Route::Chat))
        .disabled_if(|data, _| data.route == Route::Chat);
    let contacts_button = contact_icon()
        .on_click(|_, data: &mut AppState, _| data.route.goto(Route::Contacts))
        .disabled_if(|data, _| data.route == Route::Contacts);
    let settings_button = settings_icon()
        .on_click(|_, data: &mut AppState, _| data.route.goto(Route::Settings))
        .disabled_if(|data, _| data.route == Route::Settings);

    return Flex::column()
        .must_fill_main_axis(true)
        .with_spacer(10.)
        .with_child(chat_button)
        .with_spacer(10.)
        .with_child(contacts_button)
        .with_spacer(10.)
        .with_child(settings_button)
        .with_flex_spacer(1.)
        .padding(10.)
        .background(Color::BLACK);
}
