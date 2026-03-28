use dioxus::desktop::window;
use dioxus::prelude::*;

use crate::os::{App, AppMetadata, Openable};
use crate::ui::util::AppLabel;

#[component]
pub(super) fn Content(apps: Vec<App>) -> Element {
    let mut selected = use_signal(|| 0usize);
    let len = apps.len();

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

    let onkeydown = {
        let apps = apps.clone();
        move |evt: KeyboardEvent| {
            let key = evt.key();
            let down =
                matches!(key, Key::ArrowDown) || matches!(&key, Key::Character(c) if c == "j");
            let up = matches!(key, Key::ArrowUp) || matches!(&key, Key::Character(c) if c == "k");
            if down {
                selected.set((selected() + 1) % len);
                scroll_into_view(selected());
            } else if up {
                selected.set(selected().checked_sub(1).unwrap_or(len - 1));
                scroll_into_view(selected());
            } else if matches!(key, Key::Enter) {
                open(apps[selected()].clone());
            } else if matches!(key, Key::Escape) {
                window().close();
            }
        }
    };

    rsx! {
        div {
            class: "rounded-lg overflow-y-auto w-full max-h-screen",
            style: "background: var(--sidebar-background); color: var(--sidebar-foreground);",
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
                                "data-active": selected() == i,
                                onclick: {
                                    let app = app.clone();
                                    move |_| open(app.clone())
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
