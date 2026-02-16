use dioxus::prelude::*;

use crate::components::select::*;
use crate::os::App;

#[component]
pub fn MainAppPicker(
    apps: Vec<App>,
    main_app: Option<App>,
    set_main_app: Callback<Option<App>>,
) -> Element {
    let placeholder = match &main_app {
        Some(app) => app.to_string(),
        None => "(Most Recent)".to_string(),
    };
    let value: Option<Option<Option<App>>> = Some(Some(main_app));
    rsx! {
        div { class: "flex-1",
        Select::<Option<App>> {
            value,
            placeholder,
            on_value_change: move |choice: Option<Option<App>>| {
                set_main_app.call(choice.flatten());
            },
            SelectTrigger {
                SelectValue {}
            }
            SelectList {
                SelectOption::<Option<App>> {
                    value: None::<App>,
                    text_value: "(Most Recent)".to_string(),
                    index: 0usize,
                    "(Most Recent)"
                }
                for (i, app) in apps.iter().enumerate() {
                    SelectOption::<Option<App>> {
                        value: Some(app.clone()),
                        text_value: app.to_string(),
                        index: i + 1,
                        "{app}"
                    }
                }
            }
        }
        }
    }
}
