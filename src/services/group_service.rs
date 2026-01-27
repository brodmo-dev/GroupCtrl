use log::error;
use uuid::Uuid;

use crate::os::{App, AppQuery, Openable, System};
use crate::services::ConfigReader;

pub struct GroupService {
    config_reader: ConfigReader,
}

impl GroupService {
    pub fn new(config_reader: ConfigReader) -> Self {
        Self { config_reader }
    }

    pub fn open(&self, group_id: Uuid) {
        let apps = self
            .config_reader
            .read()
            .group(group_id)
            .unwrap()
            .apps()
            .clone();
        if let Ok(Some(current)) = System::current_app()
            && let Some(pos) = apps.iter().position(|app| app == &current)
        {
            let next_pos = (pos + 1) % apps.len();
            Self::open_app(&apps[next_pos]);
        } else if let Some(app) = apps.first() {
            Self::open_app(app);
        }
    }

    fn open_app(app: &App) {
        let result = app.open();
        if let Err(error) = result {
            // This can fail because the app was uninstalled, etc
            error!(
                "Could not open app '{}' due to the following error: {}",
                app, error
            );
        }
    }
}
