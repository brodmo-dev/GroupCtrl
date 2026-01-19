use crate::os::{App, AppSelection, System};

impl AppSelection for System {
    async fn select_app() -> anyhow::Result<Option<App>> {
        todo!();
    }
}
