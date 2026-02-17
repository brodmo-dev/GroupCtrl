use std::path::Path;

use dioxus::prelude::*;

use crate::os::{App, AppMetadata};

#[component]
pub fn AppLabel(app: App) -> Element {
    let icon_src = app
        .icon_path()
        .unwrap_or(Path::new("doesnotexist"))
        .display()
        .to_string();
    rsx! {
        div { class: "flex gap-2",
            div { class: "w-5 h-5 shrink-0 flex items-center justify-center",
                img { src: icon_src }
            }
            span { "{app.name()}" }
        }
    }
}
