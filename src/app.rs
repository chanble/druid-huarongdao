
use druid::{
    theme, AppLauncher, Color, Data, Lens, LocalizedString, RenderContext, Widget, WidgetExt,
    WindowDesc, Size
};
use druid::widget::prelude::*;
use druid::widget::{Label, Painter, BackgroundBrush, Flex, CrossAxisAlignment};

use num_huarongdao::num_hrd::NumHrd;

#[derive(Clone, Lens)]
pub struct AppState {
    pub hrd_data: Vec<Vec<usize>>,
    pub state: String,
}

impl Data for AppState {
    fn same(&self, other: &Self) -> bool {
        self.hrd_data == other.hrd_data
    }
}

pub fn build_app() -> impl Widget<AppState> {
    let display = Label::new(|data: &String, _env: &_| data.clone())
        .with_text_size(32.0)
        .lens(AppState::state)
        .padding(5.0);
    Flex::column()
        .with_flex_spacer(0.2)
        .with_child(display)
        .with_flex_spacer(0.2)
        .cross_axis_alignment(CrossAxisAlignment::End)
        .with_flex_child(
            flex_row(
                vec!(
                    Box::new(digit_button(7)),
                    Box::new(digit_button(8)),
                    Box::new(digit_button(9)),
                )
            ),
            1.0,
        )
        .with_spacer(1.0)
        .with_flex_child(
            flex_row(
                vec!(
                    Box::new(digit_button(4)),
                    Box::new(digit_button(5)),
                    Box::new(digit_button(6)),
                )
            ),
            1.0,
        )
        .with_spacer(1.0)
        .with_flex_child(
            flex_row(
                vec!(
                    Box::new(digit_button(1)),
                    Box::new(digit_button(2)),
                    Box::new(digit_button(3)),
                )
            ),
            1.0,
        )
}

fn flex_row<T: Data>(
    vw: Vec<Box<dyn Widget<T>>>
) -> impl Widget<T> {
    let mut flex = Flex::row();
    for x in vw {
        flex = flex.with_flex_child(x, 1.0).with_spacer(1.0);
    }
    flex
}


fn digit_button(digit: u8) -> impl Widget<AppState> {
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