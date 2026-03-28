use dioxus::desktop::tao::event::{Event, WindowEvent};
use dioxus::desktop::{
    Config, LogicalPosition, LogicalSize, WindowBuilder, use_wry_event_handler, window,
};
use dioxus::prelude::*;

use super::app_list::AppList;
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

#[component]
fn Window(apps: Vec<App>) -> Element {
    let window_id = window().id();
    use_wry_event_handler(move |event, _| {
        if let Event::WindowEvent {
            event: WindowEvent::Focused(false),
            window_id: id,
            ..
        } = event
            && *id == window_id
        {
            window().close();
        }
    });

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("../../components/sidebar/style.css") }
        div {
            onmounted: move |_| window().set_visible(true),
            AppList { apps }
        }
    }
}
