
use druid::{
    theme, AppLauncher, Color, Data, Lens, LocalizedString, RenderContext, Widget, WidgetExt,
    WindowDesc,
};
use druid::widget::prelude::*;
use druid::widget::{Label, Painter, BackgroundBrush, Flex};

use num_huarongdao::num_hrd::NumHrd;

#[derive(Clone, Lens)]
pub struct AppState {
    hrd: NumHrd,
}

impl Data for AppState {
    fn same(&self, other: &Self) -> bool {
        self.hrd == other.hrd
    }
}

impl AppState {
    pub fn new(size: u8) -> Self {
        let hrd = NumHrd::new(&size);
        Self {
            hrd,
        }
    }

    pub fn get_hrd(&self) -> &NumHrd {
        &self.hrd
    }

}

fn digit_button(digit: usize) -> impl Widget<AppState> {
    let painter = Painter::new(|ctx, _, env| {
        let bounds = ctx.size().to_rect();

        ctx.fill(bounds, &env.get(theme::BACKGROUND_LIGHT));

        if ctx.is_hot() {
            ctx.stroke(bounds.inset(-0.5), &Color::WHITE, 1.0);
        }

        if ctx.is_active() {
            ctx.fill(bounds, &Color::rgb8(0x71, 0x71, 0x71));
        }
    });

    Label::new(format!("{}", digit))
        .with_text_size(24.)
        .center()
        .background(painter)
        .expand()
}

pub fn build_hrd(hrd: NumHrd) -> impl Widget<AppState> {

    let mut column = Flex::column();
    let d2_data = hrd.as_2d_vec();

    for r in d2_data {
        let mut row = Flex::row();
        for c in r {
            row = row.with_flex_child(digit_button(c), 1.0)
            .with_spacer(1.0);
        }
        column = column.with_spacer(1.0)
            .with_flex_child(
                row,
                1.0,
            );
    }
    column
}