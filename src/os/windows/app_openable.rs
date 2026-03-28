use crate::os::{App, Openable};

mod open_app;
mod win32;

impl Openable for App {
    async fn open(id: &str) -> anyhow::Result<()> {
        open_app::open_app(id)
    }
}
