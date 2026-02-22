use dioxus::prelude::Key;
use global_hotkey::hotkey::{Code, Modifiers};

use crate::os::{Keyboard, ModifierFormat, System};

impl Keyboard for System {
    fn is_command(modifiers: Modifiers) -> bool {
        modifiers.ctrl()
    }

    fn is_quit(modifiers: Modifiers, key: Key) -> bool {
        modifiers.alt() && key == Key::F4
    }

    fn is_close(modifiers: Modifiers, key: Code) -> bool {
        modifiers.ctrl() && key == Key::F4
    }

    fn show_parts_sep() -> &'static str {
        "+"
    }

    fn show_key(_key: Code) -> Option<String> {
        None
    }

    fn show_modifier_format() -> ModifierFormat {
        [
            (Modifiers::META, "Win"),
            (Modifiers::CONTROL, "Ctrl"),
            (Modifiers::ALT, "Alt"),
            (Modifiers::SHIFT, "Shift"),
        ]
    }

    fn serde_modifier_format() -> ModifierFormat {
        [
            (Modifiers::CONTROL, "Ctrl"),
            (Modifiers::META, "Win"),
            (Modifiers::ALT, "Alt"),
            (Modifiers::SHIFT, "Shift"),
        ]
    }
}
