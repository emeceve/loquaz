use druid::{Application, Env, EventCtx};

use crate::{
    data::{app_state::AppState, state::contact_state::ContactState},
    delegate::{CONNECT_RELAY, DISCONNECT_RELAY, REMOVE_CONTACT, REMOVE_RELAY},
};

pub struct ConfigController {}

impl ConfigController {
    pub fn click_add_relay_url(ctx: &mut EventCtx, data: &mut AppState, _env: &Env) {
        // ctx.submit_command(CONNECT.with(ctx.get_external_handle()));
        data.add_relay_url();
    }

    pub fn click_copy_pk_to_clipboard(ctx: &mut EventCtx, data: &mut AppState, _env: &Env) {
        let mut clipboard = Application::global().clipboard();
        clipboard.put_string(data.user.pk.clone());
    }
    pub fn click_generate_sk(ctx: &mut EventCtx, data: &mut AppState, _env: &Env) {
        data.generate_sk();
    }
    pub fn click_restore_sk(ctx: &mut EventCtx, data: &mut AppState, _env: &Env) {
        data.restore_sk();
    }
    pub fn click_remove_relay(ctx: &mut EventCtx, data: &mut String, _env: &Env) {
        ctx.submit_command(REMOVE_RELAY.with(data.clone()));
    }
    pub fn click_connect_relay(ctx: &mut EventCtx, data: &mut String, _env: &Env) {
        ctx.submit_command(CONNECT_RELAY.with(data.clone()));
    }
    pub fn click_disconnect_relay(ctx: &mut EventCtx, data: &mut String, _env: &Env) {
        ctx.submit_command(DISCONNECT_RELAY.with(data.clone()));
    }
    pub fn click_remove_contact(ctx: &mut EventCtx, data: &mut ContactState, _env: &Env) {
        ctx.submit_command(REMOVE_CONTACT.with(data.clone()));
    }
}
