use druid::{
    theme::{PRIMARY_DARK, PRIMARY_LIGHT, WINDOW_BACKGROUND_COLOR},
    widget::{CrossAxisAlignment, Either, Flex, Label, List, Maybe, Painter, Scroll, TextBox},
    Color, Env, LensExt, RenderContext, UnitPoint, Widget, WidgetExt,
};

use crate::{
    components::button::button,
    data::{
        app_state::AppState,
        state::{
            config_state::ConfigState,
            contact_state::ContactState,
            conversation_state::{ConversationState, MessageSourceState, MessageState},
        },
    },
    pages::chat::controller::ChatController,
};

pub fn chat_tab() -> impl Widget<AppState> {
    let root = Flex::column();

    let mut lists = Flex::row().cross_axis_alignment(CrossAxisAlignment::Start);

    lists.add_flex_child(
        Scroll::new(
            List::new(chat_contact_item).lens(AppState::config.then(ConfigState::contacts)),
        )
        .vertical(),
        1.0,
    );

    let msg_text_box = TextBox::new()
        .with_placeholder("Say hello")
        .expand_width()
        .lens(AppState::msg_to_send);

    // let send_btn = Button::new("Send").on_click(ChatController::click_send_msg);
    let send_btn = button("SEND").on_click(ChatController::click_send_msg);

    lists.add_flex_child(
        Flex::column()
            .with_child(
                Label::new(|contact: &ContactState, _env: &_| format!("{}", contact.alias))
                    .lens(AppState::current_chat_contact),
            )
            .with_flex_child(
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
fn chat_conversation() -> impl Widget<ConversationState> {
    Scroll::new(List::new(|| chat_message()))
        .vertical()
        .lens(ConversationState::messages)
}

fn chat_message() -> impl Widget<MessageState> {
    Either::new(
        |data: &MessageState, env: &Env| data.source == MessageSourceState::Them,
        {
            let text = Label::raw()
                .lens(MessageState::content)
                .padding(10.)
                .background(PRIMARY_DARK)
                .rounded(10.);

            Flex::row().with_child(text).with_flex_spacer(1.)
        },
        {
            let text = Label::raw()
                .lens(MessageState::content)
                .padding(10.)
                .background(PRIMARY_LIGHT)
                .rounded(10.);
            Flex::row().with_flex_spacer(1.).with_child(text)
        },
    )
}

fn chat_contact_item() -> impl Widget<ContactState> {
    let painter = Painter::new(move |ctx, data: &ContactState, env| {
        // let selected = data.is_selected;
        let selected = true;

        let bounds = ctx.size().to_rect();

        if ctx.is_hot() {
            ctx.fill(bounds, &Color::rgb8(20, 20, 20));
        } else if selected {
            ctx.fill(bounds, &Color::BLACK);
        } else {
            ctx.fill(bounds, &env.get(WINDOW_BACKGROUND_COLOR));
        }
    });

    Flex::column()
        .with_child(Label::raw().lens(ContactState::alias))
        .with_child(
            Label::new(|pk: &String, _env: &_| {
                let mut truncate_str = String::from(pk);
                truncate_str.truncate(6);
                format!("{}...", truncate_str)
            })
            .lens(ContactState::pk),
        )
        .padding(20.)
        .background(painter)
        // .with_default_spacer()
        .on_click(ChatController::click_select_conv)
}
