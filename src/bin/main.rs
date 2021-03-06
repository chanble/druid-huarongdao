
use druid::{
    AppLauncher, LocalizedString, WindowDesc,
};

use druid_huarongdao::app::{self, AppData};

use num_huarongdao::num_hrd::{NumHrd};

fn main() {
    let hrd = NumHrd::new(&3);
    let app_state = AppData::new(hrd);
    let window = WindowDesc::new(app::make_widget)
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


