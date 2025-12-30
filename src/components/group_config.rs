use dioxus::prelude::*;
use futures_util::StreamExt;
use uuid::Uuid;

use crate::components::lists::{AppList, ListOperation};
use crate::components::util::{EditableText, HotkeyPicker};
use crate::os::{AppDialog, AppSelection};
use crate::services::ConfigService;

#[component]
pub fn GroupConfig(config_service: Signal<ConfigService>, group_id: Uuid) -> Element {
    let picked_hotkey = use_signal(|| config_service.read().group(group_id).unwrap().hotkey);
    use_effect(move || {
        config_service.write().set_hotkey(group_id, picked_hotkey());
    });

    let name = use_signal(|| config_service.read().group(group_id).unwrap().name.clone());
    use_effect(move || config_service.write().set_name(group_id, name()));

    use_app_list_change_listener(config_service, group_id);
    let apps = config_service
        .read()
        .group(group_id)
        .unwrap()
        .apps()
        .to_vec();
    rsx! {
        div {
            class: "flex flex-col gap-2",
            EditableText { text: name }
            HotkeyPicker { picked_hotkey }
            AppList { apps }
        }
    }
}

fn use_app_list_change_listener(mut config_service: Signal<ConfigService>, group_id: Uuid) {
    let app_list_change_listener = use_coroutine(
        move |mut receiver: UnboundedReceiver<ListOperation<String>>| async move {
            while let Some(cc) = receiver.next().await {
                let mut cs = config_service.write();
                match cc {
                    ListOperation::Add => {
                        if let Ok(Some(app)) = AppDialog::select_app().await {
                            cs.add_app(group_id, app)
                        }
                    }
                    ListOperation::Remove(apps) => {
                        for app_id in apps {
                            cs.remove_app(group_id, app_id);
                        }
                    }
                }
            }
        },
    );
    use_context_provider(|| app_list_change_listener.tx()); // used in the (generic) list
}
