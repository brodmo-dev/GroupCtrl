use std::rc::Rc;

use dioxus::prelude::*;

use crate::components::button::{Button, ButtonVariant};
use crate::models::Hotkey;
use crate::ui::util::use_listener;
use crate::util::is_modifier;

type ButtonHandle = Signal<Option<Rc<MountedData>>>;

#[component]
pub fn HotkeyPicker(mut hotkey: Option<Hotkey>, set_hotkey: Callback<Option<Hotkey>>) -> Element {
    let mut recording = use_signal(|| false);
    let mut handle: ButtonHandle = use_context_provider(|| Signal::new(None));
    use_record_registered(recording, set_hotkey);
    let onkeydown = move |evt: KeyboardEvent| record_unregistered(recording, set_hotkey, evt);

    let label = if recording() {
        rsx! {
            span { "Press hotkey..." }
        }
    } else {
        match hotkey {
            None => rsx! {
                span { class: "text-(--muted-text)", "No hotkey assigned" }
            },
            Some(key) => rsx! {
                span {
                    class: "gap-0.5 flex justify-center",
                    "{key}"
                }
            },
        }
    };
    let variant = if recording() {
        ButtonVariant::Secondary
    } else {
        ButtonVariant::Outline
    };
    rsx! {
        Button {
            variant,
            class: "button",
            tabindex: 0,
            onmounted: move |evt: MountedEvent| handle.set(Some(evt.data())),
            onclick: move |_| recording.set(true),
            onkeydown, // globally registered keys never make it here
            onblur: move |_| recording.set(false),
            { label }
        }
    }
}

fn blur() {
    let handle: ButtonHandle = consume_context();
    if let Some(handle) = handle() {
        spawn(async move { drop(handle.set_focus(false).await) });
    }
}

fn record_unregistered(
    mut recording: Signal<bool>,
    set_hotkey: Callback<Option<Hotkey>>,
    evt: KeyboardEvent,
) {
    let code = evt.code();
    if !recording() && code == Code::Enter {
        recording.set(true);
        return;
    }
    if !recording() || is_modifier(&code) {
        return;
    }

    set_hotkey.call(if code == Code::Escape {
        None
    } else {
        Some(Hotkey::new(evt.modifiers(), code))
    });
    blur(); // sets recording to false
}

fn use_record_registered(recording: Signal<bool>, set_hotkey: Callback<Option<Hotkey>>) {
    let mut active_recorder = use_context::<Signal<Option<UnboundedSender<Hotkey>>>>();
    let recorder = use_listener(Callback::new(move |hotkey| {
        set_hotkey.call(Some(hotkey));
        blur(); // sets recording to false
    }));
    use_effect(move || {
        active_recorder.set(if recording() {
            Some(recorder.clone())
        } else {
            None
        })
    });
}
