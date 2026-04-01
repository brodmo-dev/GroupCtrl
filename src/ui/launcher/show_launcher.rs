use dioxus::desktop::tao::event::{Event, WindowEvent};
use dioxus::desktop::{
    Config, LogicalPosition, LogicalSize, WindowBuilder, WindowCloseBehaviour,
    use_wry_event_handler, window,
};
use dioxus::prelude::*;
use log::{error, info, warn};

use super::launcher_apps::LauncherApps;
use super::launcher_state::{ACTIVE_LAUNCHER, CANCEL_RESTORE, LAUNCHER_WINDOW};
use crate::models::{Group, Identifiable};
use crate::os::{App, AppQuery, FocusedScreen, LauncherWindow, Openable, System};
use crate::ui::util::use_listener;

const WIDTH: f64 = 250.0;
const MAX_HEIGHT: f64 = 280.0;
const Y_POS: f64 = 0.4;

pub fn create_launcher_window() {
    let cfg = Config::new()
        .with_window(
            WindowBuilder::new()
                .with_visible(false)
                .with_decorations(false)
                .with_transparent(true)
                .with_always_on_top(true)
                .with_resizable(false)
                .with_inner_size(LogicalSize::new(WIDTH, MAX_HEIGHT)),
        )
        .with_custom_head(crate::custom_head())
        .with_close_behaviour(WindowCloseBehaviour::WindowHides);
    let dom = VirtualDom::new(Window);
    spawn(async move {
        let _ = window().new_window(dom, cfg).await;
    });
}

// Called from main DOM
pub fn show_launcher(group: Group) {
    if let Some(tx) = LAUNCHER_WINDOW.get() {
        let _ = tx.unbounded_send(group);
    } else {
        error!("launcher window failed to initialize");
    }
}

pub(super) fn close() {
    let mut group: Signal<Option<Group>> = consume_context();
    let mut prev_app: Signal<Option<String>> = consume_context();
    let app = prev_app.peek().clone();
    prev_app.set(None);
    ACTIVE_LAUNCHER.set(None);
    spawn(async move {
        if let Some(id) = app {
            if CANCEL_RESTORE.get().is_some() {
                info!("skipping prev app restore");
            } else {
                App::open(&id).await.ok();
            }
        }
        System::hide_launcher_window(&window());
        // reset only after window is hidden to keep LauncherApps conditional stable
        group.set(None);
    });
}

#[component]
fn Window() -> Element {
    let mut group: Signal<Option<Group>> = use_context_provider(|| Signal::new(None));
    let mut prev_app: Signal<Option<String>> = use_context_provider(|| Signal::new(None));

    let set_launcher_window = use_listener(Callback::new(move |new_group: Group| {
        let current = System::current_app().ok().flatten();
        info!(
            "showing launcher for group {}, prev_app={current:?}",
            new_group.name
        );
        set_launcher_position(&current, &new_group);
        prev_app.set(current);
        group.set(Some(new_group));
        CANCEL_RESTORE.set(None); // reset for new dialog
        System::show_launcher_window(&window());
    }));

    use_hook(|| {
        System::configure_launcher_window(&window());
        LAUNCHER_WINDOW.set(Some(set_launcher_window));
    });

    let window_id = window().id();
    use_wry_event_handler(move |event, _| {
        if let Event::WindowEvent {
            event: WindowEvent::Focused(false),
            window_id: id,
            ..
        } = event
            && *id == window_id
        {
            close();
        }
    });

    if window().is_visible() && group().is_none() {
        warn!("launcher group should not be none with visible launcher window");
    }
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("../../components/sidebar/style.css") }
        if let Some(group) = group() {
            LauncherApps { group, prev_app }
        }
    }
}

fn set_launcher_position(current: &Option<String>, group: &Group) {
    let (x, y, w, h) = current
        .as_ref()
        .filter(|id| group.apps().iter().any(|a| a.id() == **id))
        .and_then(|_| System::focused_screen())
        .unwrap_or_else(|| {
            let monitor = window().primary_monitor().unwrap();
            let s = monitor.size().to_logical::<f64>(monitor.scale_factor());
            (0.0, 0.0, s.width, s.height)
        });
    window()
        .window
        .set_outer_position(LogicalPosition::new(x + (w - WIDTH) / 2.0, y + h * Y_POS));
}
