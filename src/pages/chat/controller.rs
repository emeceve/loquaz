use druid::{Env, EventCtx};

use crate::{
    data::{
        app_state::{AppState, ChatMsg},
        contact::Contact,
        conversation::Conversation,
    },
    delegate::{SELECT_CONV, SEND_MSG, START_CHAT},
};

pub struct ChatController {}

impl ChatController {
    pub fn click_send_msg(ctx: &mut EventCtx, data: &mut AppState, _env: &Env) {
        //TODO: Change ChatMsg to Msg
        let new_msg = ChatMsg::new(&data.current_chat_contact.pk, &data.msg_to_send);
        data.push_new_msg(new_msg.clone());
        ctx.submit_command(SEND_MSG.with(new_msg));
        data.msg_to_send = "".into();
    }
    pub fn click_start_chat(ctx: &mut EventCtx, data: &mut Contact, _env: &Env) {
        ctx.submit_command(START_CHAT.with(data.pk.clone()));
    }
    pub fn click_select_conv(ctx: &mut EventCtx, data: &mut Conversation, _env: &Env) {
        ctx.submit_command(SELECT_CONV.with(data.contact.pk.clone()));
    }
}
