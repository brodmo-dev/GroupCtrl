use global_hotkey::hotkey::Modifiers;

use crate::os::App;

pub trait Formatting {
    fn modifiers() -> [(Modifiers, &'static str); 4];
}

pub trait AppSelection {
    async fn select_app() -> anyhow::Result<Option<App>>;
}

pub trait Openable {
    fn open(&self) -> anyhow::Result<()>;
}
