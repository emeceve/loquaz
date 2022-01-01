use druid::{
    widget::{Button, Flex, Label, List, Scroll, TextBox},
    Data, LensExt, Widget, WidgetExt,
};

use crate::data::{
    app_state::AppState,
    state::{config_state::ConfigState, contact_state::ContactState},
};

use super::controller::ContactsController;

pub fn contacts_tab() -> impl Widget<AppState> {
    let root = Flex::column();

    root.with_child(contacts_list()).with_child(new_contact())
}

fn config_group<T: Data, W: Widget<T> + 'static>(title: &str, w: W) -> impl Widget<T> {
    Flex::column()
        .with_child(Label::new(title))
        .with_child(w)
        .padding(10.0)
}

fn new_contact() -> impl Widget<AppState> {
    let alias_input = TextBox::new()
        .with_placeholder("Contact alias")
        .expand_width()
        .padding(5.0)
        .lens(AppState::new_contact_alias);

    let pk_input = TextBox::new()
        .with_placeholder("Contact PK(Public Key)")
        .expand_width()
        .padding(5.0)
        .lens(AppState::new_contact_pk);

    let add_btn = Button::new("Add")
        .on_click(ContactsController::click_add_contact)
        .padding(5.0);

    config_group(
        "New contact",
        Flex::row()
            .with_flex_child(alias_input, 1.0)
            .with_flex_child(pk_input, 1.0)
            .with_child(add_btn),
    )
}
fn contacts_list() -> impl Widget<AppState> {
    let list =
        Scroll::new(List::new(contact_item).lens(AppState::config.then(ConfigState::contacts)))
            .vertical();

    // Scroll::new(
    //     List::new(chat_contact_item).lens(AppState::config.then(ConfigState::contacts)),
    // )

    config_group("Contacts", list)
}

fn contact_item() -> impl Widget<ContactState> {
    let alias = Label::raw().lens(ContactState::alias).expand_width();
    let pk = Label::raw().lens(ContactState::pk).expand_width();
    let del_btn = Button::new("Delete").on_click(ContactsController::click_remove_contact);

    Flex::row()
        .with_flex_child(alias, 1.0)
        .with_flex_child(pk, 1.0)
        .with_child(del_btn)
}
