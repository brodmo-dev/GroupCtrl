use dioxus::prelude::*;
use lucide_dioxus::CircleQuestionMark;

use crate::os::{App, AppMetadata};

#[component]
pub fn AppLabel(app: App) -> Element {
    let opacity = if app.icon_path().is_some() {
        ""
    } else {
        " opacity-50"
    };
    rsx! {
        div { class: "flex gap-2 {opacity}",
            div { class: "w-5 h-5 shrink-0 flex items-center justify-center",
                match app.icon_path() {
                    Some(icon) => rsx! {
                        img { src: icon.display().to_string() }
                    },
                    None => rsx! {
                        CircleQuestionMark {}
                    },
                }
            }
            span { "{app.name()}" }
        }
    }
}
