use druid::{AppLauncher, PlatformError, WindowDesc};

mod draw_ui;
mod shape;
use crate::draw_ui::ui::ui_builder;
fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(ui_builder);
    let data = 0_u32;
    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(data)
}
