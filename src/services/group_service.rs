use std::collections::VecDeque;
use std::sync::{Arc, RwLock};
use std::thread;

use log::{error, info};
use uuid::Uuid;

use crate::models::Identifiable;
use crate::os::{App, AppObserver, AppQuery, Openable, System};
use crate::services::ConfigReader;

const MAX_HISTORY: usize = 1024; // Prevent potential memory leak

#[derive(Clone)]
pub struct GroupService {
    config_reader: ConfigReader,
    history: Arc<RwLock<VecDeque<String>>>,
}

impl GroupService {
    pub fn new(config_reader: ConfigReader) -> Self {
        let history = Arc::new(RwLock::new(VecDeque::new()));
        Self::spawn_history_writer(history.clone());
        Self {
            config_reader,
            history,
        }
    }

    fn spawn_history_writer(history: Arc<RwLock<VecDeque<String>>>) {
        let rx = System::observe_app_activations();
        thread::spawn(move || {
            for app_id in rx {
                let mut history = history.write().unwrap();
                history.retain(|aid| aid != &app_id);
                history.push_front(app_id);
                history.truncate(MAX_HISTORY);
            }
        });
    }

    /// Returns `None` if an app was opened, or `Some(apps)` if nothing could be
    /// opened and the caller should show the launcher.
    pub async fn open(&self, group_id: Uuid) -> Option<Vec<App>> {
        let group = self.config_reader.read().group(group_id).unwrap().clone();
        info!("opening group {}", group.name);
        if group.apps().len() == 1 {
            Self::open_app(&group.apps()[0]).await;
            return None;
        }
        let all_running = System::running_apps().unwrap_or_default();
        let group_running: Vec<App> = group
            .apps()
            .iter()
            .filter(|app| all_running.contains(&app.id()))
            .cloned()
            .collect();
        if let Some(app) = self
            .next_app(&group_running)
            .or_else(|| group.target.clone())
            .or_else(|| self.find_in_history(&group_running))
            .or_else(|| group_running.first().cloned())
        {
            Self::open_app(&app).await;
            None
        } else {
            Some(group.apps().clone())
        }
    }

    fn next_app(&self, apps: &[App]) -> Option<App> {
        let current_id = System::current_app().ok()??;
        let pos = apps.iter().position(|app| app.id() == current_id)?;
        let next_pos = (pos + 1) % apps.len();
        Some(apps[next_pos].clone())
    }

    fn find_in_history(&self, apps: &[App]) -> Option<App> {
        self.history
            .read()
            .unwrap()
            .iter()
            .find_map(|id| apps.iter().find(|a| a.id() == *id))
            .cloned()
    }

    async fn open_app(app: &App) {
        let result = app.open().await;
        if let Err(error) = result {
            // This can fail because the app was uninstalled, etc
            error!(
                "Could not open app '{}' due to the following error: {}",
                app, error
            );
        }
    }
}
