
use druid::{
    theme, AppLauncher, Color, Data, Lens, LocalizedString, RenderContext, Widget, WidgetExt,
    WindowDesc, Size,
};
use druid::widget::{Button, Flex, Label, Slider};

use druid_huarongdao::app::{self, AppData, HuarongDaoWidget};

use num_huarongdao::num_hrd::NumHrd;

fn main() {
    let hrd = NumHrd::new(&3);
    let app_state = AppData {
        hrd_data: hrd,
        state: format!("ddd"),
    };
    let window = WindowDesc::new(make_widget)
        .window_size((223., 300.))
        .resizable(true)
        .title(
            LocalizedString::new("calc-demo-window-title").with_placeholder("Simple Calculator"),
        );

    AppLauncher::with_window(window)
        .use_simple_logger()
        .launch(app_state)
        .expect("launch failed");
}


fn make_widget() -> impl Widget<AppData> {

    Flex::column()
        .with_flex_child(
            HuarongDaoWidget {
                cell_size: Size {
                    width: 0.0,
                    height: 0.0,
                },
            },
            1.0,
        )
}
