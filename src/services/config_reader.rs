use std::sync::{Arc, RwLock, RwLockReadGuard};

use crate::models::Config;

#[derive(Default)]
pub struct ConfigReader(Arc<RwLock<Config>>);

impl ConfigReader {
    pub fn new(config: Arc<RwLock<Config>>) -> Self {
        Self(config)
    }

    // This is not reactive and only intended for usage outside Dioxus
    pub fn read(&self) -> RwLockReadGuard<'_, Config> {
        self.0.read().unwrap()
    }
}
