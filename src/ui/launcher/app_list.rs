use dioxus::desktop::window;
use dioxus::prelude::*;

use crate::os::{App, AppMetadata, Openable};
use crate::ui::util::AppLabel;

#[component]
pub(super) fn AppList(apps: Vec<App>) -> Element {
    let open = move |app: App| {
        spawn(async move {
            let _ = app.open().await;
        });
        window().close();
    };

    // Dioxus serializes ScrollToOptions with wrong field names (vertical/horizontal
    // instead of block/inline), so scrollIntoView options are silently ignored.
    let scroll_into_view = move |i: usize| {
        document::eval(&format!(
            "document.querySelectorAll('[data-sidebar=\"menu-button\"]')[{i}]?.scrollIntoView({{block:'nearest'}})"
        ));
    };

    let mut selected_idx = use_signal(|| 0usize);
    let onkeydown = {
        let apps = apps.clone();
        move |evt: KeyboardEvent| {
            let idx = selected_idx();
            let next = (idx + 1) % apps.len();
            let prev = idx.checked_sub(1).unwrap_or(apps.len() - 1);
            let mut navigate = |to: usize| {
                selected_idx.set(to);
                scroll_into_view(to);
            };
            match evt.key() {
                Key::ArrowDown => navigate(next),
                Key::Character(c) if c == "j" => navigate(next),
                Key::ArrowUp => navigate(prev),
                Key::Character(c) if c == "k" => navigate(prev),
                Key::Enter => open(apps[idx].clone()),
                Key::Escape => window().close(),
                _ => {}
            }
        }
    };

    rsx! {
        div {
            class: "sidebar-static rounded-lg overflow-hidden max-h-screen",
            tabindex: -1,
            onmounted: move |evt| async move {
                let _ = evt.data().set_focus(true).await;
            },
            onkeydown,
            div {
                class: "sidebar-content",
                ul {
                    class: "sidebar-menu",
                    for (i, app) in apps.iter().enumerate() {
                        li {
                            key: "{app.name()}",
                            class: "sidebar-menu-item",
                            button {
                                class: "sidebar-menu-button scroll-m-1",
                                "data-sidebar": "menu-button",
                                "data-size": "default",
                                "data-active": selected_idx() == i,
                                onclick: {
                                    let my_app = app.clone();
                                    move |_| open(my_app.clone())
                                },
                                AppLabel { app: app.clone() }
                            }
                        }
                    }
                }
            }
        }
    }
}
