use super::System;
use crate::os::traits::LauncherWindow;

impl LauncherWindow for System {
    fn configure_launcher_window(_window: &dioxus::desktop::DesktopContext) {}

    fn show_launcher_window(window: &dioxus::desktop::DesktopContext) {
        window.set_visible(true);
        window.set_focus();
    }

    fn hide_launcher_window(window: &dioxus::desktop::DesktopContext) {
        window.set_visible(false);
    }
}
