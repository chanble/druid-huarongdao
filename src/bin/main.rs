
use druid::{
    theme, AppLauncher, Color, Data, Lens, LocalizedString, RenderContext, Widget, WidgetExt,
    WindowDesc,
};

use crate::app::App;
use crate::square::Square;

fn main() {
    
    let window = WindowDesc::new(|| Square { })
        .window_size((223., 300.))
        .resizable(false)
        .title(
            LocalizedString::new("calc-demo-window-title").with_placeholder("Simple Calculator"),
        );

    AppLauncher::with_window(window)
        .launch(8usize)
        .expect("launch failed");
}
