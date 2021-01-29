
use druid::{
    theme, AppLauncher, Color, Data, Lens, LocalizedString, RenderContext, Widget, WidgetExt,
    WindowDesc,
};

use druid_huarongdao::app::{AppState, build_hrd, AppWidget};

use num_huarongdao::num_hrd::NumHrd;

fn main() {
    let hrd = NumHrd::new(3);
    let app_state = AppState::new(hrd.as_2d_vec());
    let window = WindowDesc::new(|| AppWidget::new(app_state.clone()))
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
