use druid::{Application, Env, EventCtx};

use crate::data::{Contact, State};
use crate::delegate::{CONNECT, DELETE_CONTACT};

pub struct ConfigController {}

impl ConfigController {
    pub fn click_connect_ws(ctx: &mut EventCtx, data: &mut State, _env: &Env) {
        ctx.submit_command(CONNECT.with(ctx.get_external_handle()));
    }
    pub fn click_add_contact(ctx: &mut EventCtx, data: &mut State, _env: &Env) {
        data.add_contact();
    }
    pub fn click_copy_pk_to_clipboard(ctx: &mut EventCtx, data: &mut State, _env: &Env) {
        let mut clipboard = Application::global().clipboard();
        clipboard.put_string(data.public_key.clone());
    }
    pub fn click_generate_restore_sk(ctx: &mut EventCtx, data: &mut State, _env: &Env) {
        data.generate_sk();
    }

    pub fn click_delete(ctx: &mut EventCtx, data: &mut Contact, _env: &Env) {
        ctx.submit_command(DELETE_CONTACT.with(data.pk.clone()));
    }
}
