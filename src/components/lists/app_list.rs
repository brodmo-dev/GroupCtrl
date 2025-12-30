use std::collections::HashSet;

use dioxus::prelude::*;

use super::list::{List, Renderable};
use crate::models::Identifiable;
use crate::os::App;

#[component]
pub fn AppList(apps: Vec<App>) -> Element {
    let selected = use_signal(HashSet::<String>::new);
    rsx! {
        div {
            List {
                elements: apps,
                selected,
            }
        }
    }
}

impl Renderable<String> for App {
    fn render(&self) -> Element {
        rsx! {
            span { "{self.id()}" }
        }
    }
}
