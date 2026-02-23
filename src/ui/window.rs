use dioxus::desktop::window;
use dioxus::prelude::*;

use crate::components::sidebar::SidebarProvider;
use crate::components::toast::ToastProvider;
use crate::os::{App, Keyboard, Openable, System, WindowConfiguration};
use crate::ui::groups::Groups;
use crate::ui::tray_icon::{handle_tray_icon_events, setup_tray_icon};

#[component]
pub fn Window() -> Element {
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
        System::configure_window();
        setup_tray_icon()
    });
    handle_tray_icon_events();

    let onmounted = move |evt: MountedEvent| {
        window().set_decorations(true);
        window().set_focus(); // necessary on macOS due to activation policy accessory
        // Always keep focus in div so shortcuts work
        let root_handle = use_signal(|| evt.data());
        let focus_root = Callback::new(move |()| {
            spawn(async move { drop(root_handle().set_focus(true).await) });
        });
        focus_root.call(());
        use_context_provider(|| focus_root);
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
