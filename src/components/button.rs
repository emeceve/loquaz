use druid::{widget::Label, Color, Data, Insets, Widget, WidgetExt};

use crate::theme::MONO_FONT;

pub fn button<T: Data>(text: &str) -> impl Widget<T> {
    Label::new(text)
        .with_text_color(Color::BLACK)
        .with_font(MONO_FONT)
        .padding(Insets::uniform_xy(10., 5.))
        .background(Color::WHITE)
        .rounded(4.)
        .border(Color::BLACK, 2.)
        .padding(Insets::uniform(3.))
        .background(Color::WHITE)
        .rounded(5.)
}
