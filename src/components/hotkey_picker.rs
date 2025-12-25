use std::sync::Arc;

use dioxus::prelude::*;
use futures_util::stream::StreamExt;

use crate::models::Hotkey;
use crate::services::HotkeyService;

fn is_modifier(code: &Code) -> bool {
    let code_str = code.to_string();
    code_str.contains("Control")
        || code_str.contains("Meta")
        || code_str.contains("Alt")
        || code_str.contains("Shift")
}

#[component]
pub fn HotkeyPicker(mut picked_hotkey: Signal<Option<Hotkey>>) -> Element {
    let mut recording = use_signal(|| false);

    let start_recording = move |_| {
        recording.set(true);
    };

    let handle_keydown = move |evt: KeyboardEvent| {
        let code = evt.code();
        if !recording() || is_modifier(&code) {
            return;
        }
        recording.set(false);
        picked_hotkey.set(if code == Code::Escape {
            None
        } else {
            Some(Hotkey::new(evt.modifiers(), code))
        })
    };

    let mut hotkey_service = use_context::<Signal<HotkeyService>>();

    // Coroutine to receive captured hotkeys
    let hotkey_coroutine = use_coroutine(move |mut rx: UnboundedReceiver<Hotkey>| async move {
        while let Some(hotkey) = rx.next().await {
            recording.set(false);
            picked_hotkey.set(Some(hotkey));
        }
    });

    // Register/clear callback when recording state changes
    use_effect(move || {
        if recording() {
            let tx = hotkey_coroutine.tx();
            let callback = Arc::new(move |hotkey: Hotkey| {
                let _ = tx.unbounded_send(hotkey);
            });
            hotkey_service
                .write()
                .binder_mut()
                .set_recording_callback(callback);
        } else {
            hotkey_service
                .write()
                .binder_mut()
                .clear_recording_callback();
        }
    });

    let label = if recording() {
        "Recording...".to_string()
    } else {
        match picked_hotkey() {
            None => "None".to_string(),
            Some(key) => key.to_string(),
        }
    };

    let label_color = if label == "None" { "gray" } else { "black" };

    rsx! {
        div {
            onkeydown: handle_keydown,
            tabindex: 0,
            button {
                onclick: start_recording,
                style: "color: {label_color};",
                "{label}"
            }
        }
    }
}
