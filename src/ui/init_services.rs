use std::rc::Rc;
use std::sync::{Arc, RwLock};

use dioxus::prelude::*;
use global_hotkey::HotKeyState as HotkeyState;
use uuid::Uuid;

use crate::models::{Action, Config, Group, Hotkey, HotkeyEvent};
use crate::services::{ActionService, ConfigReader, ConfigService, GroupService};
use crate::ui::launcher::launcher_state::{ACTIVE_LAUNCHER, CANCEL_RESTORE};
use crate::ui::launcher::{HoldToLaunch, show_launcher, use_hold_to_launch};
use crate::ui::util::use_listener;

pub fn use_config_service() -> Signal<ConfigService> {
    let config = use_hook(|| Arc::new(RwLock::new(Config::load().unwrap_or_default())));
    let config_reader = use_hook(|| ConfigReader::new(config.clone()));
    let hold_to_launch = use_hold_to_launch(config_reader.clone());
    let group_service = use_group_service(config_reader.clone(), hold_to_launch.clone());
    let action_service = use_hook(|| ActionService::new(group_service));

    let active_recorder = use_context_provider(|| Signal::new(None::<UnboundedSender<Hotkey>>));
    let hotkey_sender = use_listener(Callback::new(move |event: HotkeyEvent| {
        hold_to_launch.cancel();
        if event.state == HotkeyState::Pressed {
            dispatch_hotkey(active_recorder, action_service.clone(), event);
        }
    }));

    use_signal(|| ConfigService::new(config, hotkey_sender))
}

fn use_group_service(config_reader: ConfigReader, hold_to_launch: HoldToLaunch) -> GroupService {
    use_hook(|| {
        let on_app_open = Rc::new(move |group_id: Uuid| hold_to_launch.start(group_id));
        let on_no_app_to_open = Rc::new(|group: Group| show_launcher(group));
        GroupService::new(config_reader, on_app_open, on_no_app_to_open)
    })
}

fn dispatch_hotkey(
    active_recorder: Signal<Option<UnboundedSender<Hotkey>>>,
    action_service: ActionService,
    event: HotkeyEvent,
) {
    if let Some(tx) = active_recorder() {
        tx.unbounded_send(event.hotkey).unwrap();
    } else if let Some((tx, active_group)) = ACTIVE_LAUNCHER.get()
        && matches!(&event.action, Action::OpenGroup { group_id } if active_group == *group_id)
    {
        let _ = tx.unbounded_send(());
    } else {
        CANCEL_RESTORE.set(Some(()));
        spawn(async move {
            action_service.execute(&event.action).await;
        });
    }
}
