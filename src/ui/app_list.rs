use std::collections::HashSet;

use dioxus::prelude::*;

use crate::models::Identifiable;
use crate::os::App;
use crate::ui::util::{AppLabel, ListMenu, use_selection};

#[component]
pub fn AppList(apps: Vec<App>) -> Element {
    let selected = use_signal(HashSet::<String>::new);
    rsx! {
        div {
            class: "sidebar-static rounded-xl",
            div {
                class: "sidebar-header",
                label { r#for: "app-list", class: "pl-1.25", "Apps" }
                ListMenu { selected }
            }
            div {
                class: "sidebar-content",
                ul {
                    id: "app-list",
                    class: "sidebar-menu",
                    for app in apps {
                        li {
                            class: "sidebar-menu-item",
                            AppRow { app, selected }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn AppRow(app: App, selected: Signal<HashSet<String>>) -> Element {
    let (is_selected, toggle) = use_selection(app.id(), selected);
    rsx! {
        button {
            class: "sidebar-menu-button",
            "data-sidebar": "menu-button",
            "data-size": "default",
            "data-active": is_selected(),
            onclick: move |e| toggle.call(e),
            AppLabel { app }
        }
    }
}
