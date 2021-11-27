use druid::{Application, Env, EventCtx};

use crate::{
    data::{app_state::AppState, state::contact_state::ContactState},
    delegate::{CONNECT, CONNECT_RELAY, DISCONNECT_RELAY, REMOVE_CONTACT, REMOVE_RELAY},
};

pub struct ContactsController {}

impl ContactsController {
    pub fn click_add_contact(ctx: &mut EventCtx, data: &mut AppState, _env: &Env) {
        data.add_contact();
    }
    pub fn click_remove_contact(ctx: &mut EventCtx, data: &mut ContactState, _env: &Env) {
        ctx.submit_command(REMOVE_CONTACT.with(data.clone()));
    }
}
