
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
    let mut flex = Flex::column()
        .with_flex_spacer(0.2)
        .with_child(display)
        .with_flex_spacer(0.2)
        .cross_axis_alignment(CrossAxisAlignment::End);

    flex = flex_vec(flex, vec!(
        vec!(0, 1, 2,),
        vec!(3, 4, 5,),
        vec!(6, 7, 8,),
    ));
    flex
}

fn flex_vec(mut flex: Flex<AppState>, v: Vec<Vec<usize>>) -> Flex<AppState> {

    for r in v {
        let mut flex_row_data: Vec<Box<dyn Widget<AppState>>> = Vec::new();
        for c in r {
            flex_row_data.push(Box::new(digit_button(c)));
        }
        flex.add_flex_child(
                flex_row(flex_row_data), 1.0,
            );
        flex.add_spacer(1.0);
    }
    flex
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