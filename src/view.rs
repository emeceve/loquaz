use druid::{
    widget::{Button, Container, CrossAxisAlignment, Flex, Label, List, Scroll, Tabs, TextBox},
    Data, EventCtx, UnitPoint, Vec2, Widget, WidgetExt,
};

use crate::data::{Contact, State, TxOrNull};

pub fn root_ui() -> impl Widget<State> {
    Tabs::new()
        .with_tab("Chat", chat_tab())
        .with_tab("Config", config_tab())
}

fn chat_tab() -> impl Widget<State> {
    let root = Flex::column();

    let mut lists = Flex::row().cross_axis_alignment(CrossAxisAlignment::Start);

    lists.add_flex_child(
        Scroll::new(List::new(chat_contact_item).lens(State::contacts)).vertical(),
        1.0,
    );

    let msg_text_box = TextBox::new()
        .with_placeholder("Say hello")
        .expand_width()
        .lens(State::msg_to_send);

    let send_btn = Button::new("Send").on_click(State::click_send_msg);

    lists.add_flex_child(
        Flex::column()
            .with_child(
                Label::new(|contact: &Contact, _env: &_| format!("{}", contact.alias))
                    .lens(State::current_chat_contact),
            )
            .with_flex_child(
                Scroll::new(List::new(|| {
                    Label::new(|msg: &String, _env: &_| format!("{}", msg))
                        .align_vertical(UnitPoint::LEFT)
                        .padding(10.0)
                        .expand()
                        .height(50.0)
                }))
                .vertical()
                .lens(State::chat_messages),
                3.0,
            )
            .with_child(
                Flex::row()
                    .with_flex_child(msg_text_box, 1.0)
                    .with_child(send_btn)
                    .padding(10.0),
            ),
        2.0,
    );

    root.with_flex_child(lists, 1.0)
}

fn chat_contact_item() -> impl Widget<Contact> {
    Flex::column()
        .with_child(Label::raw().lens(Contact::alias))
        .with_child(
            Label::new(|pk: &String, _env: &_| {
                let mut truncate_str = String::from(pk);
                truncate_str.truncate(6);
                format!("{}...", truncate_str)
            })
            .lens(Contact::pk),
        )
        .with_default_spacer()
        .on_click(Contact::click_start_chat)
}

fn config_tab() -> impl Widget<State> {
    let root = Flex::column();

    root.with_child(relay_config())
        .with_child(new_contact())
        .with_child(contacts_list())
}
fn relay_config() -> impl Widget<State> {
    let text_box = TextBox::new()
        .with_placeholder("ws://example.com")
        .expand_width()
        .lens(State::ws_url);
    let connect_btn = Button::new("Connect").on_click(State::click_connect_ws);

    config_group(
        "Relay config",
        Flex::row()
            .with_flex_child(text_box, 1.)
            .with_child(connect_btn),
    )
}
fn new_contact() -> impl Widget<State> {
    let alias_input = TextBox::new()
        .with_placeholder("Contact alias")
        .expand_width()
        .padding(5.0)
        .lens(State::new_contact_alias);

    let pk_input = TextBox::new()
        .with_placeholder("Contact PK(Public Key)")
        .expand_width()
        .padding(5.0)
        .lens(State::new_contact_pk);

    let add_btn = Button::new("Add")
        .on_click(State::click_add_contact)
        .padding(5.0);

    config_group(
        "New contact",
        Flex::row()
            .with_flex_child(alias_input, 1.0)
            .with_flex_child(pk_input, 1.0)
            .with_child(add_btn),
    )
}
fn contacts_list() -> impl Widget<State> {
    let list = Scroll::new(List::new(contact_item).lens(State::contacts)).vertical();

    config_group("Contacts", list)
}

fn contact_item() -> impl Widget<Contact> {
    let alias = Label::raw().lens(Contact::alias).expand_width();
    let pk = Label::raw().lens(Contact::pk).expand_width();
    let del_btn = Button::new("Delete").on_click(Contact::click_delete);

    Flex::row()
        .with_flex_child(alias, 1.0)
        .with_flex_child(pk, 1.0)
        .with_child(del_btn)
}

fn config_group<T: Data, W: Widget<T> + 'static>(title: &str, w: W) -> impl Widget<T> {
    Flex::column()
        .with_child(Label::new(title))
        .with_child(w)
        .padding(10.0)
}
