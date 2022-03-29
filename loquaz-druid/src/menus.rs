//! Application menus.

use druid::menu::Menu;
use druid::{platform_menus, WindowId};
use druid::{widget::Controller, Data, Env, Event, EventCtx, LocalizedString, Widget};

use crate::data::app_state::AppState;

/// The main window/app menu.
#[allow(unused_mut)]
pub(crate) fn make_menu(_: Option<WindowId>, _: &AppState, _env: &Env) -> Menu<AppState> {
    let mut menu = Menu::empty();
    #[cfg(target_os = "macos")]
    {
        menu = menu.entry(platform_menus::mac::application::default());
    }

    menu.entry(edit_menu())
}

fn edit_menu<T: Data>() -> Menu<T> {
    Menu::new(LocalizedString::new("common-menu-edit-menu"))
        .entry(platform_menus::common::copy())
        .entry(platform_menus::common::paste())
}

pub fn make_context_menu<T: Data>() -> Menu<T> {
    Menu::empty()
        // .entry(platform_menus::common::copy())
        .entry(platform_menus::common::paste())
}

pub struct ContextMenuController;

impl<T, W: Widget<T>> Controller<T, W> for ContextMenuController {
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        match event {
            Event::MouseDown(ref mouse) if mouse.button.is_right() => {
                ctx.show_context_menu(make_context_menu::<AppState>(), mouse.pos)
            }
            _ => child.event(ctx, event, data, env),
        }
    }
}
