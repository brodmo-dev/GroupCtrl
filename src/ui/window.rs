use std::rc::Rc;

use dioxus::desktop::tao::event::Event;
use dioxus::desktop::{use_wry_event_handler, window};
use dioxus::prelude::*;

use crate::components::sidebar::SidebarProvider;
use crate::components::toast::ToastProvider;
use crate::os::{Keyboard, System, WindowConfiguration};
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

    let mut root_handle = use_signal(|| None::<Rc<MountedData>>);
    let focus_root = Callback::new(move |()| {
        spawn(async move {
            if let Some(handle) = root_handle() {
                let _ = handle.set_focus(true).await;
            }
        });
    });
    use_context_provider(|| focus_root);

    use_wry_event_handler(move |event, _| {
        if matches!(event, Event::Reopen { .. }) {
            window().set_visible(true);
            window().set_focus();
            focus_root.call(());
        }
    });

    let onmounted = move |evt: MountedEvent| {
        root_handle.set(Some(evt.data()));
    };

    let onkeydown = move |evt: KeyboardEvent| {
        if System::is_quit(evt.modifiers(), evt.key()) {
            std::process::exit(0);
        } else if System::is_close(evt.modifiers(), evt.key()) {
            window().set_visible(false);
        }
    };

    rsx! {
        div {
            tabindex: -1,
            onmounted,
            onkeydown,
            ToastProvider {
                SidebarProvider {
                    Groups {}
                }
            }
        }
    }
}
