use druid::{
    text::EditableText,
    widget::{CrossAxisAlignment, Flex, Label, List, MainAxisAlignment, Scroll, TextBox},
    Data, LensExt, Widget, WidgetExt,
};

use crate::{
    components::{
        button::{button, danger_button},
        header::{header, jumbo_header},
    },
    data::{
        app_state::AppState,
        state::{config_state::ConfigState, user_state::UserState},
    },
    theme::MONO_FONT,
};

use super::controller::ConfigController;

pub fn config_tab() -> impl Widget<AppState> {
    let root = Flex::column().main_axis_alignment(MainAxisAlignment::Start);

    root.with_flex_child(
        Scroll::new(
            Flex::column()
                .cross_axis_alignment(CrossAxisAlignment::Start)
                .with_child(jumbo_header("Settings"))
                .with_spacer(20.)
                .with_child(relays_list())
                .with_child(relay_config())
                .with_spacer(20.)
                .with_child(pub_key())
                .with_spacer(20.)
                .with_child(secret_key())
                .padding(10.),
        )
        .vertical(),
        1.,
    )
}

fn secret_key() -> impl Widget<AppState> {
    let sk_input = TextBox::multiline()
        .with_placeholder("Paste your private key here to restore")
        .with_font(MONO_FONT)
        .expand_width()
        .lens(AppState::user.then(UserState::sk));

    let regenerate_button = danger_button("Generate New")
        .on_click(ConfigController::click_generate_sk)
        .fix_width(200.);

    let restore_button = button("Restore")
        .disabled_if(|data: &AppState, _| data.user.sk.len() != 64)
        .on_click(ConfigController::click_restore_sk)
        .fix_width(200.);

    let forms = Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(sk_input)
        .with_default_spacer()
        .with_child(restore_button)
        .with_default_spacer()
        .with_child(regenerate_button);

    config_group("!!! Private Key !!!", forms)
}
fn relay_config() -> impl Widget<AppState> {
    let text_box = TextBox::new()
        .with_placeholder("wss://example.com")
        .with_font(MONO_FONT)
        .expand_width()
        .lens(AppState::new_relay_ulr);
    let connect_btn = button("Add").on_click(ConfigController::click_add_relay_url);

    config_group(
        "New Relay",
        Flex::row()
            .with_flex_child(text_box, 1.)
            .with_default_spacer()
            .with_child(connect_btn),
    )
}
fn relays_list() -> impl Widget<AppState> {
    let list = Scroll::new(List::new(|| {
        let url = Label::new(|url: &String, _: &_| format!("{}", url))
            .with_font(MONO_FONT)
            .expand_width();
        let del_btn = button("Delete").on_click(ConfigController::click_remove_relay);
        let con_btn = button("Connect").on_click(ConfigController::click_connect_relay);

        // TODO: is it cool if we drop this?
        // let discon_btn = button("Disconnect").on_click(ConfigController::click_disconnect_relay);
        Flex::row()
            .with_flex_child(url, 1.0)
            .with_child(del_btn)
            .with_default_spacer()
            // .with_child(discon_btn)
            .with_child(con_btn)
            .padding((0., 0., 0., 5.))
    }))
    .vertical()
    .lens(AppState::config.then(ConfigState::relays_url));

    config_group("Relays", list)
}

fn pub_key() -> impl Widget<AppState> {
    let pk_label = TextBox::multiline()
        .with_font(MONO_FONT)
        .disabled_if(|_, _| true)
        .expand_width()
        // All this fancy shit is so we can pretty print pk
        .lens(AppState::user.then(UserState::pk.map(
            |pk| {
                if pk.len() > 0 {
                    pk.chars().enumerate().fold(String::new(), |acc, (i, c)| {
                        if i != 0 && i % 32 == 0 {
                            format!("{}\n{}", acc, c)
                        } else if i != 0 && i % 4 == 0 {
                            format!("{} {}", acc, c)
                        } else {
                            format!("{}{}", acc, c)
                        }
                    })
                } else {
                    return format!("").into();
                }
            },
            |pk, edited_pk| *pk = format!("{}", pk),
        )));

    let copy_pk_btn = button("Copy")
        .on_click(ConfigController::click_copy_pk_to_clipboard)
        .disabled_if(|data: &AppState, _| data.user.sk.len() == 0)
        .fix_width(200.);

    config_group(
        "Pub Key",
        Flex::column()
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .with_child(pk_label)
            .with_default_spacer()
            .with_child(copy_pk_btn),
    )
}

fn config_group<T: Data, W: Widget<T> + 'static>(title: &str, w: W) -> impl Widget<T> {
    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(header(title))
        .with_default_spacer()
        .with_child(w)
        .with_default_spacer()
}
