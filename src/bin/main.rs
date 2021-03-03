
use druid::{
    theme, AppLauncher, Color, Data, Lens, LocalizedString, RenderContext, Widget, WidgetExt,
    WindowDesc, Size, TimerToken,
};
use druid::widget::{Button, Flex, Label, FlexParams, CrossAxisAlignment};
use rand::prelude::*;
use druid_huarongdao::app::{self, AppData, HuarongDaoWidget, State};

use num_huarongdao::num_hrd::{NumHrd, Direction};

fn main() {
    let hrd = NumHrd::new(&4);
    let app_state = AppData::new(hrd);
    let window = WindowDesc::new(make_widget)
        .window_size((223., 300.))
        .resizable(true)
        .title(
            LocalizedString::new("fifteen-puzzle-window-title").with_placeholder("数字华容道-- by chanble_cn@163.com"),
        );

    AppLauncher::with_window(window)
        .use_simple_logger()
        .launch(app_state)
        .expect("launch failed");
}


fn make_widget() -> impl Widget<AppData> {

    Flex::column()
        .with_flex_child(
            Label::new("数字华容道")
                .with_text_size(52.0)
                .padding(5.0),
            1.0,
        )
        .with_child(
            Flex::row()
            .with_child(Label::new("用时："))
            .with_child(Label::new(|data: &String, _env: &_| data.clone()).lens(AppData::duration))
            .with_child(Label::new("步数："))
            .with_child(Label::new(|data: &String, _env: &_| data.clone()).lens(AppData::steps))
            
        )
        .with_flex_child(
            HuarongDaoWidget {
                timer_id: TimerToken::INVALID,
                cell_size: Size {
                    width: 0.0,
                    height: 0.0,
                },
            },
            2.0,
        )
        .with_child(
            Button::new("开始游戏")
            .padding(15.0).on_click(|_ctx, data: &mut AppData, _env| {
                for i in 0..500 {
                    let mut rng = rand::thread_rng();
                    let r = rng.gen_range(0..4);
                    let direction = match r {
                        0 => Direction::Top,
                        1 => Direction::Bottom,
                        2 => Direction::Left,
                        _ => Direction::Right,
                    };
                    data.hrd_data.zero_move(&direction).unwrap();
                }
                println!("shuffle over");
                data.state = State::Ready;
                if data.state == State::Ready {
                    data.start();
                }
                _ctx.request_paint();
            }),
        )
        .with_child(
            Label::new(|data: &String, _env: &_| data.clone())
                .with_text_size(24.0)
                .with_text_color(Color::rgb8(0xf0, 0xf0, 0xea))
                .padding(3.0)
                .lens(AppData::tip)
        )
}
