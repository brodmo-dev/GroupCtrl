use std::sync::mpsc::{self, Receiver};

use crate::os::{AppObserver, System};

impl AppObserver for System {
    fn observe_app_activations() -> Receiver<String> {
        todo!()
    }
}
