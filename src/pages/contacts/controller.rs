use druid::{Env, EventCtx};

use crate::{
    data::{app_state::AppState, state::contact_state::ContactState},
    delegate::REMOVE_CONTACT,
};

pub struct ContactsController {}

impl ContactsController {
    // TODO why isn't this command based?
    pub fn click_add_contact(ctx: &mut EventCtx, data: &mut AppState, _env: &Env) {
        data.add_contact();
    }
    pub fn click_remove_contact(ctx: &mut EventCtx, data: &mut ContactState, _env: &Env) {
        ctx.submit_command(REMOVE_CONTACT.with(data.clone()));
    }
}
