use std::rc::Rc;

use dioxus::prelude::*;

#[component]
pub fn EditableText(text: Signal<String>) -> Element {
    #[allow(clippy::redundant_closure)]
    let mut draft = use_signal(|| text());
    let mut input_handle = use_signal(|| None::<Rc<MountedData>>);
    let onkeydown = move |evt: KeyboardEvent| {
        match evt.key() {
            Key::Enter => text.set(draft()),
            Key::Escape => draft.set(text()),
            _ => return,
        }
        drop(input_handle().unwrap().set_focus(false));
    };
    rsx! {
        input {
            class: "input input-ghost input-xs font-bold text-sm w-full",
            value: "{draft}",
            onmounted: move |evt| input_handle.set(Some(evt.data())),
            oninput: move |evt| draft.set(evt.value()),
            onblur: move |_| draft.set(text()),
            onkeydown,
        }
    }
}
