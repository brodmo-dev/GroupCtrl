use std::collections::HashSet;
use std::hash::Hash;

use dioxus::prelude::*;

#[derive(Clone)]
pub enum ListOperation<I>
where
    I: Clone + Eq + Hash + 'static,
{
    Add, // adding is interactive
    Remove(HashSet<I>),
}

#[component]
pub(super) fn ListMenu<I>(selected: Signal<HashSet<I>>) -> Element
where
    I: Clone + Eq + Hash + 'static,
{
    let sender = use_context::<UnboundedSender<ListOperation<I>>>();
    let my_sender = sender.clone();
    let add = move |_| drop(sender.unbounded_send(ListOperation::Add));
    let remove = move |_| drop(my_sender.unbounded_send(ListOperation::Remove(selected())));

    rsx! {
        div {
            class: "flex gap-1",
            button {
                class: "btn btn-xs btn-square",
                onclick: add,
                "+"
            }
            button {
                class: "btn btn-xs btn-square",
                disabled: selected().is_empty(),
                onclick: remove,
                "-"
            }
        }
    }
}
