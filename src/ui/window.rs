use dioxus::desktop::tao::event::Event;
use dioxus::desktop::{use_wry_event_handler, window};
use dioxus::prelude::*;

use crate::components::sidebar::SidebarProvider;
use crate::components::toast::ToastProvider;
use crate::os::{System, WindowConfiguration};
use crate::ui::groups::Groups;
use crate::ui::launcher::create_launcher_window;
use crate::ui::tray_icon::{handle_tray_icon_events, setup_tray_icon};

#[component]
pub fn Window() -> Element {
    use_hook(|| {
        System::configure_window();
        create_launcher_window();
        setup_tray_icon()
    });
    handle_tray_icon_events();

    use_wry_event_handler(move |event, _| {
        if matches!(event, Event::Reopen { .. }) {
            window().set_visible(true);
            window().set_focus();
        }
    });

    rsx! {
        ToastProvider {
            SidebarProvider {
                Groups {}
            }
        }
    }
}
