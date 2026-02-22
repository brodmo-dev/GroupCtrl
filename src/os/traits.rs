use std::path::{Path, PathBuf};

use global_hotkey::hotkey::{Code, Modifiers};

use crate::os::App;

pub type ModifierFormat = [(Modifiers, &'static str); 4];

pub trait Keyboard {
    fn is_command(modifiers: Modifiers) -> bool;
    fn is_quit(modifiers: Modifiers, key: Code) -> bool;
    fn is_close(modifiers: Modifiers, key: Code) -> bool;
    fn show_parts_sep() -> &'static str;
    fn show_key(key: Code) -> Option<String>;
    fn show_modifier_format() -> ModifierFormat;
    fn serde_modifier_format() -> ModifierFormat;
}

pub trait ConfigDir {
    fn config_dir() -> PathBuf;
}

pub trait AppQuery {
    fn current_app() -> anyhow::Result<Option<String>>;
}

pub trait AppSelection {
    async fn select_app() -> anyhow::Result<Option<App>>;
}

pub trait Openable {
    async fn open(&self) -> anyhow::Result<()>;
}

pub trait AppMetadata {
    fn name(&self) -> &str;
    fn icon_path(&self) -> Option<&Path>;
}

pub trait AppObserver {
    fn observe_app_activations() -> std::sync::mpsc::Receiver<String>;
}

pub trait WindowConfiguration {
    fn configure_window();
}
