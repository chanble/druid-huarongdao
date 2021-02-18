
use druid::{
    theme, AppLauncher, Color, Data, Lens, LocalizedString, RenderContext, Widget, WidgetExt,
    WindowDesc,
};

use druid_huarongdao::app::{self, AppState};

use num_huarongdao::num_hrd::NumHrd;

fn main() {
    let hrd = NumHrd::new(&1);
    let mut hrd_data1: Vec<Vec<usize>> = Vec::new();
    let mut hrd_data_row: Vec<usize> = Vec::new();
    hrd_data_row.push(0);
    hrd_data_row.push(1);
    hrd_data_row.push(2);
    hrd_data1.push(hrd_data_row);
    let app_state = AppState {
        hrd_data: hrd_data1,
        state: format!("ddd"),
    };
    let window = WindowDesc::new(app::build_app)
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
