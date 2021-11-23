use druid::{Env, EventCtx};

use crate::data::{ChatMsg, Contact, State};
use crate::delegate::{SEND_MSG, START_CHAT};

pub struct ChatController {}

impl ChatController {
    pub fn click_send_msg(ctx: &mut EventCtx, data: &mut State, _env: &Env) {
        let new_msg = ChatMsg::new(&data.current_chat_contact.pk, &data.msg_to_send);
        data.push_new_msg(new_msg.clone());
        ctx.submit_command(SEND_MSG.with(new_msg));
        data.msg_to_send = "".into();
    }
    pub fn click_start_chat(ctx: &mut EventCtx, data: &mut Contact, _env: &Env) {
        ctx.submit_command(START_CHAT.with(data.pk.clone()));
    }
}
