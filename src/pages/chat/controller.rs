use druid::{
    im::Vector,
    keyboard_types::Key,
    widget::{Controller, Scroll, TextBox},
    Data, Env, Event, EventCtx, Rect, UpdateCtx, Widget,
};

use crate::{
    data::{
        app_state::AppState,
        state::{contact_state::ContactState, conversation_state::NewMessage},
    },
    delegate::{SELECT_CONV, SEND_MSG, START_CHAT},
};

pub struct ChatController {}

impl ChatController {
    pub fn click_send_msg(ctx: &mut EventCtx, data: &mut AppState, _env: &Env) {
        // TODO if we disable the send box when no convo is selected we don't need this guard
        if data.selected_conv.is_some() {
            let pk = data.selected_conv.clone().unwrap().contact.pk;
            let content = data.msg_to_send.clone();
            //TODO: Use chat new message instead
            ctx.submit_command(SEND_MSG.with(NewMessage::new(&pk, &content)));

            data.msg_to_send = "".into();
        }
    }
    pub fn click_start_chat(ctx: &mut EventCtx, data: &mut ContactState, _env: &Env) {
        ctx.submit_command(START_CHAT.with(data.pk.clone()));
    }
    pub fn click_select_conv(ctx: &mut EventCtx, data: &mut ContactState, _env: &Env) {
        ctx.submit_command(SELECT_CONV.with(data.pk.clone()));
    }
}

pub struct OnEnterController;

impl<W: Widget<AppState>> Controller<AppState, W> for OnEnterController {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut AppState,
        env: &Env,
    ) {
        match event {
            Event::KeyUp(k_e) => {
                if k_e.key == Key::Enter {
                    // TODO if we disable the send box when no convo is selected we don't need this guard
                    if data.selected_conv.is_some() {
                        let pk = data.selected_conv.clone().unwrap().contact.pk;
                        let content = data.msg_to_send.clone();
                        let trimmed = content.trim_end();
                        ctx.submit_command(SEND_MSG.with(NewMessage::new(&pk, &trimmed)));

                        data.msg_to_send = "".into();
                    }
                    ctx.set_handled()
                }
            }
            _ => {}
        }

        if (!ctx.is_handled()) {
            child.event(ctx, event, data, env);
        }
    }
}

pub struct ConversationScrollController;

impl<T: Data, W: Widget<Vector<T>>> Controller<Vector<T>, Scroll<Vector<T>, W>>
    for ConversationScrollController
{
    fn update(
        &mut self,
        child: &mut Scroll<Vector<T>, W>,
        ctx: &mut UpdateCtx,
        old_data: &Vector<T>,
        data: &Vector<T>,
        env: &Env,
    ) {
        // TODO: Ideally we only do this when a new message arrives (could do a Command for this?)
        if old_data.len() != data.len() {
            // This scrolls to the bottom, but it scrolls to the bottom of the messages BEFORE the new one is added.
            // So since it can't measure the new height, it can't scroll down far enough.
            child.scroll_by((0.0, f64::INFINITY).into());
            ctx.request_paint();
        }
        child.update(ctx, old_data, data, env);
    }
}
