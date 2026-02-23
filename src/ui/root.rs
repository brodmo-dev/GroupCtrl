use std::collections::HashSet;
use std::rc::Rc;
use std::sync::{Arc, RwLock};

use dioxus::desktop::window;
use dioxus::prelude::*;
use uuid::Uuid;

use super::group_config::GroupConfig;
use super::util::{ListMenu, ListOperation, use_listener, use_selection};
use crate::components::sidebar::*;
use crate::components::toast::ToastProvider;
use crate::models::{Config, Hotkey, Identifiable};
use crate::os::{App, Keyboard, Openable, System, WindowConfiguration};
use crate::services::{ActionService, ConfigReader, ConfigService};
use crate::ui::tray_icon::{handle_tray_icon_events, setup_tray_icon};

#[component]
pub fn Root() -> Element {
    // restore focus for hot reload quality of life
    #[cfg(all(debug_assertions, target_os = "macos"))]
    use_effect(move || {
        if let Some(id) = crate::PREVIOUS_APP.get() {
            spawn(async move {
                let _ = App::from(id.clone()).open().await;
            });
        }
    });

    use_hook(|| {
        System::configure_window();
        setup_tray_icon()
    });
    handle_tray_icon_events();

    let config_service = use_config_service();
    let selected = use_signal(HashSet::<Uuid>::new);
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

    let onmounted = move |evt: MountedEvent| {
        window().set_decorations(true);
        provide_unfocus_callback().set(Some(evt.data()));
    };
    let onkeydown = move |evt: KeyboardEvent| handle_window_shortcuts(evt.modifiers(), evt.key());
    let border_pad_val = if cfg!(target_os = "macos") {
        "1px" // compensate for macOS window border
    } else {
        "0px"
    };
    rsx! {
        div {
            tabindex: -1,
            onmounted,
            onkeydown,
            ToastProvider {
            SidebarProvider {
                Sidebar {
                    style: "padding-left: {border_pad_val};",
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

// Dioxus shortcuts only work if focus is in window -> restore focus to root div to unfocus
fn provide_unfocus_callback() -> Signal<Option<Rc<MountedData>>> {
    let root_handle = use_signal(|| None::<Rc<MountedData>>);
    use_context_provider(|| {
        Callback::new(move |()| {
            if let Some(handle) = root_handle() {
                spawn(async move { drop(handle.set_focus(true).await) });
            }
        })
    });
    root_handle
}

fn handle_window_shortcuts(modifiers: Modifiers, key: Key) {
    if System::is_quit(modifiers, key.clone()) {
        std::process::exit(0);
    } else if System::is_close(modifiers, key) {
        window().set_visible(false);
    }
}
