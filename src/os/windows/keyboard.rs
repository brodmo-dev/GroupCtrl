use global_hotkey::hotkey::{Code, Modifiers};

use crate::os::{Keyboard, ModifierFormat, System};

impl Keyboard for System {
    fn is_multi_select(modifiers: Modifiers) -> bool {
        modifiers.ctrl()
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
