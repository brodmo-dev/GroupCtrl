use objc2_app_kit::NSWorkspace;

use crate::os::{AppQuery, System};

impl AppQuery for System {
    fn current_app() -> anyhow::Result<Option<String>> {
        Ok(NSWorkspace::sharedWorkspace()
            .frontmostApplication()
            .and_then(|app| app.bundleIdentifier())
            .map(|bid| bid.to_string()))
    }

    fn running_apps() -> anyhow::Result<Vec<String>> {
        Ok(NSWorkspace::sharedWorkspace()
            .runningApplications()
            .iter()
            .filter_map(|app| app.bundleIdentifier().map(|bid| bid.to_string()))
            .collect())
    }
}
