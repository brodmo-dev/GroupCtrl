use crate::os::{AppQuery, System};

impl AppQuery for System {
    fn current_app() -> anyhow::Result<Option<String>> {
        Ok(None)
    }

    fn running_apps() -> anyhow::Result<Vec<String>> {
        Ok(vec![])
    }
}
