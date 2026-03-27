use dioxus::desktop::tao::event::{Event, WindowEvent};
use dioxus::desktop::{
    Config, LogicalPosition, LogicalSize, WindowBuilder, use_wry_event_handler, window,
};
use dioxus::prelude::*;
use futures::StreamExt;
use futures::channel::mpsc;

use crate::os::{App, AppMetadata, Openable};
use crate::ui::util::AppLabel;

const WIDTH: f64 = 250.0;
const ROW_HEIGHT: usize = 32;
const PAD: usize = 4;
const GAP: usize = 4;
pub async fn show(apps: Vec<App>) {
    let (tx, mut rx) = mpsc::unbounded::<Option<App>>();
    let height = ((ROW_HEIGHT + GAP) * apps.len() - GAP + 2 * PAD) as f64;

    let dom = VirtualDom::new_with_props(
        Launcher,
        LauncherProps {
            apps,
            sender: Sender(tx),
        },
    );
    let monitor = window()
        .primary_monitor()
        .or_else(|| window().current_monitor())
        .unwrap();
    let scale = monitor.scale_factor();
    let screen = monitor.size().to_logical::<f64>(scale);

    let cfg = Config::new()
        .with_window(
            WindowBuilder::new()
                .with_visible(false)
                .with_decorations(false)
                .with_transparent(true)
                .with_always_on_top(true)
                .with_resizable(false)
                .with_inner_size(LogicalSize::new(WIDTH, height))
                .with_position(LogicalPosition::new(
                    (screen.width - WIDTH) / 2.0,
                    (screen.height - height) / 2.0,
                )),
        )
        .with_custom_head(crate::custom_head());

    window().new_window(dom, cfg).await;

    if let Some(Some(app)) = rx.next().await {
        let _ = app.open().await;
    }
}

#[derive(Clone)]
struct Sender(mpsc::UnboundedSender<Option<App>>);

impl PartialEq for Sender {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

#[derive(Clone, PartialEq, Props)]
struct LauncherProps {
    apps: Vec<App>,
    sender: Sender,
}

#[component]
fn Launcher(props: LauncherProps) -> Element {
    let window_id = window().id();
    let sender_for_focus = props.sender.0.clone();
    use_wry_event_handler(move |event, _| {
        if let Event::WindowEvent {
            event: WindowEvent::Focused(false),
            window_id: id,
            ..
        } = event
            && *id == window_id
        {
            let _ = sender_for_focus.unbounded_send(None);
            window().close();
        }
    });

    let mut selected = use_signal(|| 0usize);
    let apps = &props.apps;
    let sender = &props.sender.0;

    let send = {
        let sender = sender.clone();
        move |app: Option<App>| {
            let _ = sender.unbounded_send(app);
            window().close();
        }
    };

    let onkeydown = {
        let apps = apps.clone();
        let send = send.clone();
        move |evt: KeyboardEvent| {
            let len = apps.len();
            let key = evt.key();
            let down =
                matches!(key, Key::ArrowDown) || matches!(&key, Key::Character(c) if c == "j");
            let up = matches!(key, Key::ArrowUp) || matches!(&key, Key::Character(c) if c == "k");
            if down {
                selected.set((selected() + 1) % len);
            } else if up {
                selected.set(selected().checked_sub(1).unwrap_or(len - 1));
            } else if matches!(key, Key::Enter) {
                send(Some(apps[selected()].clone()));
            } else if matches!(key, Key::Escape) {
                send(None);
            }
        }
    };

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("../components/sidebar/style.css") }
        div {
            class: "rounded-lg outline-none overflow-hidden",
            style: "background: var(--sidebar-background); color: var(--sidebar-foreground);",
            tabindex: -1,
            onmounted: move |evt| {
                spawn(async move {
                    window().set_visible(true);
                    let _ = evt.data().set_focus(true).await;
                });
            },
            onkeydown,
            div {
                class: "sidebar-content",
                ul {
                    class: "sidebar-menu",
                    for (i, app) in apps.iter().enumerate() {
                        li {
                            key: "{app.name()}",
                            class: "sidebar-menu-item",
                            button {
                                class: "sidebar-menu-button",
                                "data-sidebar": "menu-button",
                                "data-size": "default",
                                "data-active": selected() == i,
                                onclick: {
                                    let app = app.clone();
                                    let send = send.clone();
                                    move |_| send(Some(app.clone()))
                                },
                                AppLabel { app: app.clone() }
                            }
                        }
                    }
                }
            }
        }
    }
}
