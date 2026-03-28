use dioxus::desktop::tao::event::{Event, WindowEvent};
use dioxus::desktop::{use_wry_event_handler, window};
use dioxus::prelude::*;

use super::app_list::AppList;
use crate::os::App;

#[component]
pub(super) fn Window(apps: Vec<App>) -> Element {
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
            onmounted: move |_| {
                window().set_visible(true);
            },
            AppList { apps }
        }
    }
}
