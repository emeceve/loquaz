use druid::{
    lens::Unit,
    widget::{Svg, SvgData, WidgetExt},
    LensExt,
};

use super::NostrWidget;
use crate::{data::app_state::AppState, theme};

use druid::{kurbo::BezPath, widget::prelude::*, Affine, Color, KeyOrValue, Size};

#[derive(Copy, Clone)]
pub enum PaintOp {
    Fill,
}

#[derive(Clone)]
pub struct SvgIcon {
    svg_path: &'static str,
    svg_size: Size,
    op: PaintOp,
}

impl SvgIcon {
    pub fn new<T: Data>(
        &self,
        scale_to_size: impl Into<Size>,
        color: KeyOrValue<Color>,
        hover_color: Option<KeyOrValue<Color>>,
    ) -> impl Widget<T> {
        let to_size = scale_to_size.into();
        let bez_path = BezPath::from_svg(self.svg_path).expect("Failed to parse SVG");
        let scale = Affine::scale_non_uniform(
            to_size.width / self.svg_size.width,
            to_size.height / self.svg_size.height,
        );
        if let Some(hover_color) = hover_color {
            Icon::new(self.op, bez_path, to_size, scale)
                .with_color(color)
                .with_hover_color(hover_color)
                // .highlight_on_hover()
                .boxed()
        } else {
            Icon::new(self.op, bez_path, to_size, scale)
                .with_color(color)
                .boxed()
        }
    }
}

#[derive(Clone)]
pub struct Icon {
    op: PaintOp,
    bez_path: BezPath,
    size: Size,
    scale: Affine,
    color: KeyOrValue<Color>,
    hover_color: KeyOrValue<Color>,
}

impl Icon {
    pub fn new(op: PaintOp, bez_path: BezPath, size: Size, scale: Affine) -> Self {
        Icon {
            op,
            bez_path,
            size,
            scale,
            color: theme::ICON_COLOR.into(),
            hover_color: theme::ICON_COLOR.into(),
        }
    }

    pub fn set_color(&mut self, color: impl Into<KeyOrValue<Color>>) {
        self.color = color.into();
    }

    pub fn with_color(mut self, color: impl Into<KeyOrValue<Color>>) -> Self {
        self.color = color.into();
        self
    }

    pub fn with_hover_color(mut self, color: impl Into<KeyOrValue<Color>>) -> Self {
        self.hover_color = color.into();
        self
    }

    // pub fn highlight_on_hover<D: Data>(self) -> impl Widget<D> {
    //     // on_hover method is from the NostrWidget "extension" trait
    //     let base_color = self.color.clone();
    //     let hover_color = self.hover_color.clone();
    //     self.on_hover(
    //         move |ctx, _, this, _env| {
    //             if !ctx.is_disabled() {
    //                 this.set_color(hover_color.clone());
    //                 ctx.request_paint();
    //             }
    //         },
    //         move |ctx, _, this, _env| {
    //             if !ctx.is_disabled() {
    //                 this.set_color(base_color.clone());
    //                 ctx.request_paint();
    //             }
    //         },
    //     )
    // }
}

impl<T: Data> Widget<T> for Icon {
    fn event(&mut self, _ctx: &mut EventCtx, _ev: &Event, _data: &mut T, _env: &Env) {}

    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _ev: &LifeCycle, _data: &T, _env: &Env) {}

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &T, _data: &T, _env: &Env) {}

    fn layout(&mut self, _ctx: &mut LayoutCtx, bc: &BoxConstraints, _data: &T, _env: &Env) -> Size {
        bc.constrain(self.size)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, _data: &T, env: &Env) {
        let color = self.color.resolve(env);
        let hover_color = self.hover_color.resolve(env);
        let disabled = ctx.is_disabled();
        let hovered = ctx.is_hot();
        ctx.with_save(|ctx| {
            ctx.transform(self.scale);
            match self.op {
                PaintOp::Fill => {
                    match (disabled, hovered) {
                        (true, _) => ctx.fill(&self.bez_path, &hover_color),
                        (false, false) => ctx.fill(&self.bez_path, &color),
                        (false, true) => ctx.fill(&self.bez_path, &hover_color),
                    }
                    // if let Some(hover_color) = self.hover_color {
                    //     ctx.fill(&self.bez_path, &color)
                    // }
                }
            }
        });
    }
}

// const CHAT_ICON: &'static str = include_str!("../../assets/pixelarticons_message.svg");

pub static CHAT_ICON: SvgIcon = SvgIcon {
    svg_path: include_str!("../../assets/pixelarticons_message.svg"),
    svg_size: Size::new(48., 48.),
    op: PaintOp::Fill,
};

pub static CONTACT_ICON: SvgIcon = SvgIcon {
    svg_path: include_str!("../../assets/pixelarticons_contact.svg"),
    svg_size: Size::new(48., 48.),
    op: PaintOp::Fill,
};

pub static SETTINGS_ICON: SvgIcon = SvgIcon {
    svg_path: include_str!("../../assets/pixelarticons_sliders.svg"),
    svg_size: Size::new(48., 48.),
    op: PaintOp::Fill,
};

pub fn chat_icon() -> impl Widget<AppState> {
    CHAT_ICON.new(
        (24.0, 24.0),
        Color::grey8(0x72).into(),
        Some(Color::WHITE.into()),
    )
}

pub fn contact_icon() -> impl Widget<AppState> {
    CONTACT_ICON.new(
        (24.0, 24.0),
        Color::grey8(0x72).into(),
        Some(Color::WHITE.into()),
    )
}

pub fn settings_icon() -> impl Widget<AppState> {
    SETTINGS_ICON.new(
        (24.0, 24.0),
        Color::grey8(0x72).into(),
        Some(Color::WHITE.into()),
    )
}
