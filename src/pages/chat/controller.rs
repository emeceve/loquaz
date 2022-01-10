use druid::{Env, EventCtx};

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
        if data.selected_conv.is_some() {
            let pk = data.selected_conv.clone().unwrap().contact.pk;
            let content = data.msg_to_send.clone();
            //TODO: Use chat new message instead
            // let content = data.selected_conv.unwrap().new_message;
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
