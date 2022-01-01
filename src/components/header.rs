use druid::{widget::Label, Color, Data, Insets, Widget, WidgetExt};

use crate::theme::{MONO_FONT, MONO_FONT_BOLD, TEXT_4XL};

pub fn jumbo_header<T: Data>(text: &str) -> impl Widget<T> {
    Label::new(text)
        .with_font(MONO_FONT_BOLD)
        .with_text_size(TEXT_4XL)
}

pub fn header<T: Data>(text: &str) -> impl Widget<T> {
    Label::new(text).with_font(MONO_FONT_BOLD)
}
