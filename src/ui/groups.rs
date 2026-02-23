use std::collections::HashSet;
use std::sync::{Arc, RwLock};

use dioxus::prelude::*;
use uuid::Uuid;

use super::group_config::GroupConfig;
use super::util::{ListMenu, ListOperation, use_listener, use_selection};
use crate::components::sidebar::*;
use crate::models::{Config, Hotkey, Identifiable};
use crate::services::{ActionService, ConfigReader, ConfigService};

#[component]
pub fn Groups() -> Element {
    let config_service = use_config_service();
    let mut selected = use_signal(HashSet::<Uuid>::new);
    let in_creation_group = use_signal(|| None::<Uuid>);
    use_group_list_listener(config_service, selected, in_creation_group);

    let active_group = use_memo(move || {
        if selected().len() == 1 {
            selected().iter().next().copied()
        } else {
            None
        }
    });
    let groups = config_service.read().config().groups().clone();

    let border_pad_val = if cfg!(target_os = "macos") {
        "1px" // compensate for macOS window border
    } else {
        "0px"
    };
    rsx! {
        Sidebar {
            style: "padding-left: {border_pad_val};",
            onclick: move |_| selected.write().clear(),
            collapsible: SidebarCollapsible::None,
            SidebarHeader {
                label { r#for: "group-list", class: "pl-1", "Groups" }
                ListMenu { selected }
            }
            SidebarContent {
                SidebarMenu { id: "group-list",
                    for group in groups {
                        GroupMenuItem {
                            key: "{group.id()}",
                            group_id: group.id(),
                            name: group.name.clone(),
                            selected,
                        }
                    }
                }
            }
        }
        SidebarInset {
            style: "padding-bottom: {border_pad_val}; padding-right: {border_pad_val};",
            if let Some(group_id) = active_group() {
                GroupConfig {
                    key: "{group_id}",
                    config_service,
                    group_id,
                    in_creation_group
                }
            }
        }
    }
}

#[component]
fn GroupMenuItem(group_id: Uuid, name: String, selected: Signal<HashSet<Uuid>>) -> Element {
    let (is_active, toggle) = use_selection(group_id, selected);

    rsx! {
        SidebarMenuItem {
            SidebarMenuButton {
                is_active: is_active(),
                onclick: move |e| toggle.call(e),
                span { "{name}" }
            }
        }
    }
}

fn use_config_service() -> Signal<ConfigService> {
    let config = use_hook(|| Arc::new(RwLock::new(Config::load().unwrap_or_default())));
    let config_reader = use_hook(|| ConfigReader::new(config.clone()));
    let action_service = use_hook(|| ActionService::new(config_reader.clone()));

    let active_recorder = use_context_provider(|| Signal::new(None::<UnboundedSender<Hotkey>>));
    let hotkey_sender = use_listener(Callback::new(move |(hotkey, action)| {
        if let Some(sender) = active_recorder() {
            sender.unbounded_send(hotkey).unwrap();
        } else {
            let service = action_service.clone();
            spawn(async move {
                service.execute(&action).await;
            });
        }
    }));

    use_signal(|| ConfigService::new(config, hotkey_sender))
}

fn use_group_list_listener(
    mut config_service: Signal<ConfigService>,
    mut selected: Signal<HashSet<Uuid>>,
    mut in_creation_group: Signal<Option<Uuid>>,
) {
    use_listener(Callback::new(move |list_operation: ListOperation<Uuid>| {
        selected.write().clear();
        match list_operation {
            ListOperation::Add => {
                let name = unique_group_name(&config_service.read(), "New Group");
                let group_id = config_service.write().add_group(name);
                selected.write().insert(group_id);
                in_creation_group.set(Some(group_id));
            }
            ListOperation::Remove(group_id) => {
                config_service.write().remove_group(group_id);
            }
        }
    }));
}

fn unique_group_name(config_service: &ConfigService, base: &str) -> String {
    let config = config_service.config();
    let names: Vec<&str> = config.groups().iter().map(|g| g.name.as_str()).collect();
    if !names.contains(&base) {
        return base.to_string();
    }
    let mut n = 2;
    loop {
        let candidate = format!("{} {}", base, n);
        if !names.contains(&candidate.as_str()) {
            return candidate;
        }
        n += 1;
    }
}
