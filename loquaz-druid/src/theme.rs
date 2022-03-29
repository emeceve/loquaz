use druid::{Color, Data, Env, FontDescriptor, FontFamily, FontWeight, Key};

// pub const ICON_COLOR: Key<Color> = Key::new("app.icon-color");
pub const ICON_COLOR: Color = Color::WHITE;
pub const ICON_COLOR_2: Color = Color::RED;

const DARKBLUE: Color = Color::rgb8(0x06, 0x50, 0xb6);
pub const COLOR_DARKBLUE: Key<Color> = Key::new("nostr.darkblue");

const RED: Color = Color::rgb8(0xec, 0x66, 0x64);
pub const COLOR_RED: Key<Color> = Key::new("nostr.red");

pub const MONO_FONT: Key<FontDescriptor> = Key::new("nostr.mono_font");
pub const MONO_FONT_BOLD: Key<FontDescriptor> = Key::new("nostr.mono_font_bold");
pub const TEXT_4XL: Key<f64> = Key::new("nostr.text_4xl");

pub fn set_env<T: Data>() -> impl Fn(&mut Env, &T) + 'static {
    |env, _state| {
        // env.set(druid::theme::BORDER_DARK, AA);
        // env.set(druid::theme::PRIMARY_LIGHT, BLUE);
        // env.set(druid::theme::PLACEHOLDER_COLOR, TW_GRAY_400);

        // env.set(druid::theme::CURSOR_COLOR, Color::BLACK);
        // env.set(druid::theme::SELECTION_COLOR, BITCOIN_ORANGE);

        // FIXME: this will panic without Inter!
        // env.set(
        //     druid::theme::UI_FONT,
        //     FontDescriptor::new(FontFamily::new_unchecked("Inter")).with_size(13.0),
        // );

        env.set(COLOR_RED, RED);

        env.set(druid::theme::BACKGROUND_LIGHT, Color::BLACK);
        env.set(druid::theme::BORDER_DARK, Color::WHITE);
        env.set(druid::theme::TEXTBOX_BORDER_RADIUS, 5.);
        env.set(druid::theme::TEXTBOX_BORDER_WIDTH, 2.);
        env.set(druid::theme::TEXTBOX_INSETS, 10.);
        env.set(TEXT_4XL, 17.);
        env.set(
            druid::theme::UI_FONT,
            FontDescriptor::new(FontFamily::SYSTEM_UI).with_size(13.),
        );
        env.set(
            MONO_FONT,
            FontDescriptor::new(FontFamily::MONOSPACE).with_size(13.),
        );
        env.set(
            MONO_FONT_BOLD,
            FontDescriptor::new(FontFamily::MONOSPACE)
                .with_size(13.)
                .with_weight(FontWeight::BOLD),
        );
    }
}
