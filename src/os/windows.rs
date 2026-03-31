mod app;
mod app_observer;
mod app_openable;
mod app_query;
mod app_selection;
mod config_dir;
mod focused_screen;
mod keyboard;

pub use app::App;

use crate::os::WindowConfiguration;

pub struct System;

impl WindowConfiguration for System {
    fn configure_window() {}
}
