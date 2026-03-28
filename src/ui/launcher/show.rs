use dioxus::desktop::{Config, LogicalPosition, LogicalSize, WindowBuilder, window};
use dioxus::prelude::*;

use super::window::{Window, WindowProps};
use crate::os::App;

const WIDTH: f64 = 250.0;
const MAX_HEIGHT: f64 = 280.0;
const Y_POS: f64 = 0.4;

pub async fn show(apps: Vec<App>) {
    let monitor = window()
        .primary_monitor()
        .or_else(|| window().current_monitor())
        .unwrap();
    let screen = monitor.size().to_logical::<f64>(monitor.scale_factor());
    let cfg = Config::new()
        .with_window(
            WindowBuilder::new()
                .with_visible(false)
                .with_decorations(false)
                .with_transparent(true)
                .with_always_on_top(true)
                .with_resizable(false)
                .with_inner_size(LogicalSize::new(WIDTH, MAX_HEIGHT))
                .with_position(LogicalPosition::new(
                    (screen.width - WIDTH) / 2.0,
                    screen.height * Y_POS,
                )),
        )
        .with_custom_head(crate::custom_head());
    let dom = VirtualDom::new_with_props(Window, WindowProps { apps });
    window().new_window(dom, cfg).await;
}
