use std::thread;

use dioxus::desktop::tao::event::{Event, WindowEvent};
use dioxus::desktop::{
    Config, LogicalPosition, LogicalSize, WindowBuilder, use_wry_event_handler, window,
};
use dioxus::prelude::*;
use futures::executor::block_on;

use super::launcher_apps::LauncherApps;
use super::launcher_state::{ACTIVE_LAUNCHER, PRE_LAUNCHER_APP};
use crate::models::Group;
use crate::os::{App, AppQuery, Openable, System};

const WIDTH: f64 = 250.0;
const MAX_HEIGHT: f64 = 280.0;
const Y_POS: f64 = 0.4;

pub fn show_launcher(group: Group) {
    PRE_LAUNCHER_APP.set(System::current_app().ok().flatten());
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
    let dom = VirtualDom::new_with_props(Window, WindowProps { group });
    spawn(async move {
        let _ = window().new_window(dom, cfg).await;
    });
}

pub(super) fn close() {
    let prev_app = PRE_LAUNCHER_APP.get();
    PRE_LAUNCHER_APP.set(None);
    ACTIVE_LAUNCHER.set(None);
    window().close();
    if let Some(id) = prev_app {
        thread::spawn(move || {
            block_on(App::open(&id)).ok();
        });
    }
}

#[component]
fn Window(group: Group) -> Element {
    let window_id = window().id();
    use_wry_event_handler(move |event, _| {
        if let Event::WindowEvent {
            event: WindowEvent::Focused(false),
            window_id: id,
            ..
        } = event
            && *id == window_id
        {
            close();
        }
    });

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("../../components/sidebar/style.css") }
        div {
            onmounted: move |_| {
                window().set_visible(true);
                window().set_focus();
            },
            LauncherApps { group }
        }
    }
}
