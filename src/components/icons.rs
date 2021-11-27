use druid::widget::{Svg, SvgData};

const CHAT_ICON: &'static str = include_str!("../../assets/pixelarticons_message.svg");
const CONTACT_ICON: &'static str = include_str!("../../assets/pixelarticons_contact.svg");
const SETTINGS_ICON: &'static str = include_str!("../../assets/pixelarticons_sliders.svg");

fn import_svg(icon_str: &'static str) -> Svg {
    let svg_data = match icon_str.parse::<SvgData>() {
        Ok(svg) => svg,
        Err(_err) => SvgData::default(),
    };

    Svg::new(svg_data)
}

pub fn chat_icon() -> Svg {
    import_svg(CHAT_ICON)
}

pub fn contact_icon() -> Svg {
    import_svg(CONTACT_ICON)
}

pub fn settings_icon() -> Svg {
    import_svg(SETTINGS_ICON)
}
