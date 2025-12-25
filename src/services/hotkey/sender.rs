use std::sync::{Arc, Mutex};

use dioxus::prelude::UnboundedSender;

use crate::models::Hotkey;

#[derive(Clone, Default)]
pub struct SharedHotkeySender(Arc<Mutex<Option<UnboundedSender<Hotkey>>>>);

impl SharedHotkeySender {
    pub fn set(&self, sender: Option<UnboundedSender<Hotkey>>) {
        *self.0.lock().unwrap() = sender;
    }

    pub(super) fn get(&self) -> Option<UnboundedSender<Hotkey>> {
        self.0.lock().unwrap().clone()
    }
}
