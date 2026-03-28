use dioxus::prelude::*;
use lucide_dioxus::CornerDownLeft;

use super::show::{ACTIVE_LAUNCHER, close};
use crate::models::Group;
use crate::os::{App, AppMetadata, Openable};
use crate::ui::util::{AppLabel, use_listener};

#[component]
pub(super) fn AppList(group: Group) -> Element {
    let open = move |app: App| {
        spawn(async move {
            let _ = app.open().await;
        });
        close();
    };

    // Dioxus serializes ScrollToOptions with wrong field names (vertical/horizontal
    // instead of block/inline), so scrollIntoView options are silently ignored.
    let scroll_into_view = move |i: usize| {
        document::eval(&format!(
            "document.querySelectorAll('[data-sidebar=\"menu-button\"]')[{i}]?.scrollIntoView({{block:'nearest'}})"
        ));
    };

    let mut selected_idx = use_signal(|| 0usize);
    let apps = group.apps().clone();
    let len = apps.len();
    let mut navigate = move |to: usize| {
        selected_idx.set(to);
        scroll_into_view(to);
    };
    let mut select_next = move || navigate((selected_idx() + 1) % len);
    let mut select_prev = move || navigate(selected_idx().checked_sub(1).unwrap_or(len - 1));
    let tx = use_listener(Callback::new(move |()| select_next()));
    use_hook(|| *ACTIVE_LAUNCHER.write().unwrap() = Some(tx));
    let onkeydown = {
        let apps = apps.clone();
        move |evt: KeyboardEvent| match evt.key() {
            Key::ArrowDown => select_next(),
            Key::Character(c) if c == "j" => select_next(),
            Key::ArrowUp => select_prev(),
            Key::Character(c) if c == "k" => select_prev(),
            Key::Enter => open(apps[selected_idx()].clone()),
            Key::Escape => close(),
            _ => {}
        }
    };

    rsx! {
        div {
            class: "sidebar-static rounded-lg overflow-hidden max-h-screen outline-none",
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
                                if selected_idx() == i {
                                    CornerDownLeft {
                                        class: "ml-auto !size-3",
                                        color: "var(--muted-text)",
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
