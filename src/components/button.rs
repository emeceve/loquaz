use druid::{widget::Label, Color, Data, Insets, TextAlignment, Widget, WidgetExt};

use crate::theme::{COLOR_RED, MONO_FONT};

pub fn button<T: Data>(text: &str) -> impl Widget<T> {
    Label::new(text)
        .with_text_color(Color::BLACK)
        .with_font(MONO_FONT)
        .center()
        .padding(Insets::uniform_xy(10., 5.))
        .background(Color::WHITE)
        .rounded(4.)
        .border(Color::BLACK, 2.)
        .padding(Insets::uniform(3.))
        .background(Color::WHITE)
        .rounded(5.)
}

pub fn danger_button<T: Data>(text: &str) -> impl Widget<T> {
    Label::new(text)
        .with_text_color(Color::BLACK)
        .with_font(MONO_FONT)
        .center()
        .padding(Insets::uniform_xy(10., 5.))
        .background(COLOR_RED)
        .rounded(4.)
        .border(Color::BLACK, 2.)
        .padding(Insets::uniform(3.))
        .background(COLOR_RED)
        .rounded(5.)
}
