use dioxus::desktop::trayicon::{
    Icon, MouseButton, MouseButtonState, TrayIcon, TrayIconBuilder, TrayIconEvent, menu,
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
        setup_tray_icon() // hook stores return value
    });

    let mut is_visible = use_signal(|| false);
    use_tray_icon_event_handler(move |evt| {
        if let TrayIconEvent::Click {
            button: MouseButton::Left,
            button_state,
            ..
        } = evt
        {
            match button_state {
                // Dioxus always sets the window to visible on left click up, so we capture
                // visibility on Down and act on Up to work around it.
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

fn setup_tray_icon() -> TrayIcon {
    let tray_menu = menu::Menu::new();
    tray_menu
        .append(&menu::PredefinedMenuItem::quit(None))
        .unwrap();
    let icon = Icon::from_rgba(
        include_bytes!("../../assets/tray-icon.rgba").to_vec(),
        128,
        128,
    )
    .expect("tray icon parse failed");
    TrayIconBuilder::new()
        .with_menu(Box::new(tray_menu))
        .with_menu_on_left_click(false)
        .with_icon(icon)
        .with_icon_as_template(true)
        .build()
        .expect("tray icon build failed")
}
