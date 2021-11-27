use druid::{
    widget::{Button, CrossAxisAlignment, Either, Flex, Label, List, Maybe, Scroll, TextBox},
    Lens, LensExt, UnitPoint, Widget, WidgetExt,
};

use crate::{
    data::{
        app_state::AppState,
        config::Config,
        contact::Contact,
        conversation::{Conversation, Msg},
    },
    pages::chat::controller::ChatController,
};

// struct ConversationLens;

// impl Lens<Option<Conversation>, Conversation> for ConversationLens {
//     fn with<R, F: FnOnce(&Option<Conversation>) -> R>(&self, data: &Option<Conversation>, f: F) -> R {
//         f(&data.unwrap())
//     }

//     fn with_mut<R, F: FnOnce(&mut Option<Conversation>) -> R>(&self, data: &mut Option<Conversation>, f: F) -> R {
//         f(&mut data.unwrap())
//     }
// }

// one sec let's comment this out going to try something else real quick

pub fn chat_tab() -> impl Widget<AppState> {
    let root = Flex::column();

    let mut lists = Flex::row().cross_axis_alignment(CrossAxisAlignment::Start);

    lists.add_flex_child(
        Scroll::new(List::new(chat_contact_item).lens(AppState::config.then(Config::contacts)))
            .vertical(),
        1.0,
    );

    let msg_text_box = TextBox::new()
        .with_placeholder("Say hello")
        .expand_width()
        .lens(AppState::msg_to_send);

    let send_btn = Button::new("Send").on_click(ChatController::click_send_msg);

    lists.add_flex_child(
        Flex::column()
            .with_child(
                Label::new(|contact: &Contact, _env: &_| format!("{}", contact.alias))
                    .lens(AppState::current_chat_contact),
            )
            .with_flex_child(
                //                Either::new(
                //                    |data: &AppState, _env| match data.selected_conv {
                //                        Some(_) => true,
                //                        None => false,
                //                    },
                //                    chat_conversation().lens(AppState::selected_conv.map(
                //                        |conv| conv.clone().unwrap(),
                //                        |_mutconv, _updatedconv| conv.clone().unwrap(),
                //                    )),
                //                    Label::new("False"),
                //
                Maybe::new(|| chat_conversation(), || Label::new("False"))
                    .lens(AppState::selected_conv),
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
fn chat_conversation() -> impl Widget<Conversation> {
    Scroll::new(List::new(|| {
        Label::new(|msg: &Msg, _env: &_| format!("{}", msg.content))
            .align_vertical(UnitPoint::LEFT)
            .padding(10.0)
            .expand()
            .height(50.0)
    }))
    .vertical()
    .lens(Conversation::messages)
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
        .on_click(ChatController::click_select_conv)
}
