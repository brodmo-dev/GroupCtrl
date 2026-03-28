use std::time::Duration;

use dioxus::core::Task;
use dioxus::prelude::*;
use tokio::time::sleep;
use uuid::Uuid;

use super::show_launcher::{ACTIVE_LAUNCHER, show_launcher};
use crate::services::ConfigReader;

pub fn use_hold_to_launch(config_reader: ConfigReader) -> HoldToLaunch {
    let show_launcher_task: Signal<Option<Task>> = use_signal(|| None);
    HoldToLaunch {
        config_reader,
        show_launcher_task,
    }
}

#[derive(Clone)]
pub struct HoldToLaunch {
    config_reader: ConfigReader,
    show_launcher_task: Signal<Option<Task>>,
}

impl HoldToLaunch {
    pub fn start(&self, group_id: Uuid) {
        self.cancel();
        let reader = self.config_reader.clone();
        let task = spawn(async move {
            sleep(Duration::from_millis(200)).await;
            if ACTIVE_LAUNCHER.read().unwrap().is_none() {
                let group = reader.read().group(group_id).unwrap().clone();
                show_launcher(group);
            }
        });
        let mut hold_task = self.show_launcher_task;
        hold_task.set(Some(task));
    }

    pub fn cancel(&self) {
        let mut hold_task = self.show_launcher_task;
        if let Some(task) = hold_task.take() {
            task.cancel();
        }
    }
}
