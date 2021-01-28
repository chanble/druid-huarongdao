
use druid::{
    theme, AppLauncher, Color, Data, Lens, LocalizedString, RenderContext, Widget, WidgetExt,
    WindowDesc,
};

use druid_huarongdao::app::{AppState, build_hrd};

fn main() {
    let app = AppState::new(3);
    let data = app.get_hrd().clone();
    let window = WindowDesc::new(move || build_hrd(data))
        .window_size((223., 300.))
        .resizable(true)
        .title(
            LocalizedString::new("calc-demo-window-title").with_placeholder("Simple Calculator"),
        );

    AppLauncher::with_window(window)
        .launch(app)
        .expect("launch failed");
}
