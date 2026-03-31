use dioxus::desktop::trayicon::{
    Icon, MouseButton, MouseButtonState, TrayIcon, TrayIconBuilder, TrayIconEvent, menu,
};
use dioxus::desktop::{use_tray_icon_event_handler, window};

pub(super) fn setup_tray_icon() -> TrayIcon {
    let tray_menu = menu::Menu::new();
    tray_menu
        .append(&menu::PredefinedMenuItem::quit(None))
        .unwrap();
    let icon = Icon::from_rgba(
        include_bytes!("../../assets/icons/tray-icon.rgba").to_vec(),
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

pub(super) fn handle_tray_icon_events() {
    use_tray_icon_event_handler(move |evt| {
        if let TrayIconEvent::Click {
            button: MouseButton::Left,
            button_state: MouseButtonState::Down,
            ..
        } = evt
        {
            if window().is_visible() {
                window().set_visible(false);
            } else {
                window().set_visible(true);
                window().set_focus();
            }
        }
    });
}
