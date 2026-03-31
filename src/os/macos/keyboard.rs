use dioxus::prelude::Key;
use global_hotkey::hotkey::{Code, Modifiers};

use crate::os::{Keyboard, ModifierFormat, System};

impl Keyboard for System {
    fn is_command(modifiers: Modifiers) -> bool {
        modifiers.meta()
    }

    fn is_quit(modifiers: Modifiers, key: Key) -> bool {
        modifiers.meta() && key == Key::Character("q".to_string())
    }

    fn is_close(modifiers: Modifiers, key: Key) -> bool {
        modifiers.meta() && key == Key::Character("w".to_string())
    }

    fn show_parts_sep() -> &'static str {
        ""
    }

    fn show_key(key: Code) -> Option<String> {
        let symbol = match key {
            Code::Enter => "↩",
            Code::Backspace => "⌫",
            Code::Delete => "⌦",
            Code::Escape => "⎋",
            Code::Tab => "⇥",
            Code::Space => "␣",
            Code::ArrowLeft => "←",
            Code::ArrowRight => "→",
            Code::ArrowUp => "↑",
            Code::ArrowDown => "↓",
            Code::PageUp => "⇞",
            Code::PageDown => "⇟",
            Code::Home => "↖",
            Code::End => "↘",
            Code::CapsLock => "⇪",
            _ => return None,
        };
        Some(symbol.to_string())
    }

    fn show_modifier_format() -> ModifierFormat {
        [
            (Modifiers::CONTROL, "⌃"),
            (Modifiers::ALT, "⌥"),
            (Modifiers::SHIFT, "⇧"),
            (Modifiers::META, "⌘"),
        ]
    }

    fn serde_modifier_format() -> ModifierFormat {
        [
            (Modifiers::META, "Cmd"),
            (Modifiers::ALT, "Opt"),
            (Modifiers::CONTROL, "Ctrl"),
            (Modifiers::SHIFT, "Shift"),
        ]
    }
}
