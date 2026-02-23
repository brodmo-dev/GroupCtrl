use std::rc::Rc;

use dioxus::desktop::window;
use dioxus::prelude::*;

use crate::components::sidebar::SidebarProvider;
use crate::components::toast::ToastProvider;
use crate::os::{App, Keyboard, Openable, System, WindowConfiguration};
use crate::ui::group_list::GroupList;
use crate::ui::tray_icon::{handle_tray_icon_events, setup_tray_icon};

#[component]
pub fn Root() -> Element {
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
        provide_unfocus_callback().set(Some(evt.data()));
    };
    let onkeydown = move |evt: KeyboardEvent| handle_window_shortcuts(evt.modifiers(), evt.key());
    rsx! {
        div {
            tabindex: -1,
            onmounted,
            onkeydown,
            ToastProvider {
                SidebarProvider {
                    GroupList {}
                }
            }
        }
    }
}

// Dioxus shortcuts only work if focus is in window -> restore focus to root div to unfocus
fn provide_unfocus_callback() -> Signal<Option<Rc<MountedData>>> {
    let root_handle = use_signal(|| None::<Rc<MountedData>>);
    use_context_provider(|| {
        Callback::new(move |()| {
            if let Some(handle) = root_handle() {
                spawn(async move { drop(handle.set_focus(true).await) });
            }
        })
    });
    root_handle
}

fn handle_window_shortcuts(modifiers: Modifiers, key: Key) {
    if System::is_quit(modifiers, key.clone()) {
        std::process::exit(0);
    } else if System::is_close(modifiers, key) {
        window().set_visible(false);
    }
}
