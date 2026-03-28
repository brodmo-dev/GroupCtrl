use dioxus::desktop::tao::event::{Event, WindowEvent};
use dioxus::desktop::{use_wry_event_handler, window};
use dioxus::prelude::*;

use super::content::Content;
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
            class: "h-full overflow-hidden outline-none",
            tabindex: -1,
            onmounted: move |evt| {
                spawn(async move {
                    window().set_visible(true);
                    let _ = evt.data().set_focus(true).await;
                });
            },
            Content { apps }
        }
    }
}
