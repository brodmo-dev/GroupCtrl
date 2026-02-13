use std::fmt::{Display, Formatter};

use global_hotkey::hotkey::{Code, HotKey as GlobalHotkey, Modifiers};

use crate::models::hotkey_conversion::show_hotkey_parts;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Hotkey {
    pub(super) mods: Modifiers,
    pub(super) key: Code,
}

impl Hotkey {
    pub fn new(mods: Modifiers, key: Code) -> Hotkey {
        Self { mods, key }
    }

    pub fn global_hotkey(self) -> GlobalHotkey {
        GlobalHotkey::new(Some(self.mods), self.key)
    }

    pub fn show_parts(&self) -> Vec<String> {
        show_hotkey_parts(self)
    }
}

impl Display for Hotkey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.show_parts().join("+"))
    }
}
