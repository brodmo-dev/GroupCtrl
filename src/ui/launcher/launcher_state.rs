use std::sync::RwLock;

use dioxus::hooks::UnboundedSender;

use crate::models::Group;

// Since we're in a different DOM than in the config pane we can't use Signals to communicate
pub static LAUNCHER_WINDOW: State<UnboundedSender<Group>> = State::new();
pub static ACTIVE_LAUNCHER: State<UnboundedSender<()>> = State::new();

pub struct State<T>(RwLock<Option<T>>);

impl<T> State<T> {
    const fn new() -> Self {
        Self(RwLock::new(None))
    }

    pub fn set(&self, value: Option<T>) {
        *self.0.write().unwrap() = value;
    }
}

impl<T: Clone> State<T> {
    pub fn get(&self) -> Option<T> {
        self.0.read().unwrap().clone()
    }
}
