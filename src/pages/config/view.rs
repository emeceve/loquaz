use druid::{
    widget::{Button, Flex, Label, List, Scroll, TextBox},
    Data, Widget, WidgetExt,
};

use crate::data::{Contact, State};

use super::controller::ConfigController;

pub fn config_tab() -> impl Widget<State> {
    let root = Flex::column();

    root.with_child(key_config())
        .with_child(relay_config())
        .with_child(new_contact())
        .with_child(contacts_list())
}

fn key_config() -> impl Widget<State> {
    let sk_input = TextBox::new()
        .with_placeholder("Restore or generate your secret key")
        .expand_width()
        .lens(State::secret_key);
    let pk_label = TextBox::new()
        .disabled_if(|_, _| true)
        .expand_width()
        .lens(State::public_key);

    let generate_or_restore_btn =
        Button::new("Generate/Restore").on_click(ConfigController::click_generate_restore_sk);

    let copy_pk_btn = Button::new("Copy")
        .on_click(ConfigController::click_copy_pk_to_clipboard)
        .disabled_if(|data, _| data.public_key.len() == 0);

    let forms = Flex::column()
        .with_child(
            Flex::row()
                .with_flex_child(sk_input, 1.)
                .with_child(generate_or_restore_btn),
        )
        .with_child(
            Flex::row()
                .with_flex_child(pk_label, 1.)
                .with_child(copy_pk_btn),
        );

    config_group("Generate/Restore Keys", forms)
}
fn relay_config() -> impl Widget<State> {
    let text_box = TextBox::new()
        .with_placeholder("ws://example.com")
        .expand_width()
        .lens(State::ws_url);
    let connect_btn = Button::new("Connect").on_click(ConfigController::click_connect_ws);

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
        .on_click(ConfigController::click_add_contact)
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
    //TODO abstract method call in controller
    let del_btn = Button::new("Delete").on_click(ConfigController::click_delete);

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
