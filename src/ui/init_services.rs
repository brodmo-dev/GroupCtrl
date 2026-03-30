use std::rc::Rc;
use std::sync::{Arc, RwLock};

use dioxus::prelude::*;
use global_hotkey::HotKeyState as HotkeyState;
use uuid::Uuid;

use crate::models::{Config, Group, Hotkey, HotkeyEvent};
use crate::services::{ActionService, ConfigReader, ConfigService, GroupService};
use crate::ui::launcher::launcher_state::{ACTIVE_LAUNCHER, CANCEL_RESTORE};
use crate::ui::launcher::{show_launcher, use_hold_to_launch};
use crate::ui::util::use_listener;

pub fn use_config_service() -> Signal<ConfigService> {
    let config = use_hook(|| Arc::new(RwLock::new(Config::load().unwrap_or_default())));
    let config_reader = use_hook(|| ConfigReader::new(config.clone()));
    let hold_to_launch = use_hold_to_launch(config_reader.clone());
    let action_service = use_hook(|| {
        let my_hold = hold_to_launch.clone();
        let on_app_open = Rc::new(move |group_id: Uuid| my_hold.start(group_id));
        let on_no_app_to_open = Rc::new(|group: Group| show_launcher(group));
        let group_service =
            GroupService::new(config_reader.clone(), on_app_open, on_no_app_to_open);
        ActionService::new(group_service)
    });

    let active_recorder = use_context_provider(|| Signal::new(None::<UnboundedSender<Hotkey>>));
    let hotkey_sender = use_listener(Callback::new(move |event: HotkeyEvent| {
        hold_to_launch.cancel();
        if event.state == HotkeyState::Pressed {
            if let Some(tx) = active_recorder() {
                tx.unbounded_send(event.hotkey).unwrap();
            } else if let Some(tx) = ACTIVE_LAUNCHER.get() {
                tx.unbounded_send(()).unwrap();
            } else {
                CANCEL_RESTORE.set(Some(()));
                let service = action_service.clone();
                spawn(async move {
                    service.execute(&event.action).await;
                });
            }
        }
    }));

    use_signal(|| ConfigService::new(config, hotkey_sender))
}
