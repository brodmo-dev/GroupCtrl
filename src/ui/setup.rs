use dioxus::desktop::trayicon::{
    MouseButton, MouseButtonState, TrayIconEvent, default_tray_icon, init_tray_icon,
};
use dioxus::desktop::{use_tray_icon_event_handler, window};
use dioxus::prelude::*;

use crate::os::{App, Openable, System, WindowConfiguration};

pub(super) fn setup_window() {
    // restore focus for hot reload quality of life
    #[cfg(all(debug_assertions, target_os = "macos"))]
    use_effect(move || {
        if let Some(id) = crate::PREVIOUS_APP.get() {
            spawn(async move {
                let _ = App::from(id.clone()).open().await;
            });
        }
    });

    use_hook(|| {
        window().set_decorations(true);
        System::configure_window();
        init_tray_icon(default_tray_icon(), None);
    });

    let mut is_visible = use_signal(|| false);
    use_tray_icon_event_handler(move |evt| {
        // Dioxus always sets the window to visible on left click up, so we capture
        // visibility on Down and act on Up to work around it.
        if let TrayIconEvent::Click {
            button: MouseButton::Left,
            button_state,
            ..
        } = evt
        {
            match button_state {
                MouseButtonState::Down => {
                    is_visible.set(window().is_visible());
                }
                MouseButtonState::Up => {
                    if is_visible() {
                        spawn(async move {
                            window().set_visible(false);
                        });
                    }
                }
            }
        }
    });
}
