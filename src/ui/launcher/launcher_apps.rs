use dioxus::prelude::*;
use lucide_dioxus::CornerDownLeft;

use super::launcher_state::{ACTIVE_LAUNCHER, PRE_LAUNCHER_APP};
use super::show_launcher::close;
use crate::models::{Group, Identifiable};
use crate::os::{App, AppQuery, Openable, System};
use crate::ui::util::{AppLabel, use_listener};

#[component]
pub(super) fn LauncherApps(group: Group) -> Element {
    let open = move |app: App| {
        PRE_LAUNCHER_APP.set(None);
        let id = app.id();
        spawn(async move {
            let _ = App::open(&id).await;
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

    if group.apps().is_empty() {
        return rsx! { NoApps { message: "No apps assigned to group" } };
    }
    let running_apps = System::running_apps().unwrap_or_default();
    let launch_apps: Vec<App> = group
        .apps()
        .iter()
        .filter(|app| !running_apps.contains(&app.id()))
        .cloned()
        .collect();
    if launch_apps.is_empty() {
        return rsx! { NoApps { message: "All apps already running" } };
    }
    let mut selected_idx = use_signal(|| 0usize);
    let len = launch_apps.len();
    let mut navigate = move |to: usize| {
        selected_idx.set(to);
        scroll_into_view(to);
    };
    let mut select_next = move || navigate((selected_idx() + 1) % len);
    let mut select_prev = move || navigate(selected_idx().checked_sub(1).unwrap_or(len - 1));
    let launcher_cycle = use_listener(Callback::new(move |()| select_next()));
    use_hook(|| ACTIVE_LAUNCHER.set(Some(launcher_cycle)));
    let onkeydown = {
        let my_apps = launch_apps.clone();
        move |evt: KeyboardEvent| match evt.key() {
            Key::ArrowDown => select_next(),
            Key::Character(c) if c == "j" => select_next(),
            Key::ArrowUp => select_prev(),
            Key::Character(c) if c == "k" => select_prev(),
            Key::Enter => open(my_apps[selected_idx()].clone()),
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
                    for (i, app) in launch_apps.into_iter().enumerate() {
                        li {
                            key: "{app.id()}",
                            class: "sidebar-menu-item",
                            AppRow {
                                app: app.clone(),
                                is_selected: selected_idx() == i,
                                onclick: move |_| open(app.clone()),
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn AppRow(app: App, is_selected: bool, onclick: EventHandler<MouseEvent>) -> Element {
    rsx! {
        button {
            class: "sidebar-menu-button scroll-m-1",
            "data-sidebar": "menu-button",
            "data-size": "default",
            "data-active": is_selected,
            onclick,
            AppLabel { app: app.clone() }
            if is_selected {
                CornerDownLeft {
                    class: "ml-auto !size-3",
                    color: "var(--muted-text)",
                }
            }
        }
    }
}

#[component]
fn NoApps(message: String) -> Element {
    rsx! {
        div {
            class: "sidebar-static rounded-lg outline-none",
            tabindex: -1,
            onmounted: move |evt| async move {
                let _ = evt.data().set_focus(true).await;
            },
            onkeydown: move |evt: KeyboardEvent| {
                if evt.key() == Key::Escape { close(); }
            },
            p {
                class: "p-3 text-sm text-center",
                color: "var(--muted-text)",
                "{message}"
            }
        }
    }
}
