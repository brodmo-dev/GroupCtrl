use global_hotkey::hotkey::{Code, Modifiers};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use super::Hotkey;
use crate::os::{KeyboardBehavior, ModifierFormat, System};

const SERDE_SEP: &str = "+";
const KEY_PREFIXES: [&str; 4] = ["Key", "Digit", "Arrow", ""];

pub(super) fn show_hotkey_parts(hotkey: &Hotkey) -> Vec<String> {
    hotkey_to_string_vec(hotkey.mods, hotkey.key, System::gui_modifier_format())
}

impl Serialize for Hotkey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let vec = hotkey_to_string_vec(self.mods, self.key, System::serde_modifier_format());
        serializer.serialize_str(&vec.join(SERDE_SEP))
    }
}

impl<'de> Deserialize<'de> for Hotkey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let parts: Vec<&str> = s.split(SERDE_SEP).collect();
        let (mod_parts, key_part) = parts.split_at(parts.len() - 1);
        let mods = parse_mods(mod_parts, System::serde_modifier_format());
        let key = parse_key(key_part[0]);
        Ok(Hotkey::new(mods, key))
    }
}

fn hotkey_to_string_vec(
    mods: Modifiers,
    key: Code,
    modifier_format: ModifierFormat,
) -> Vec<String> {
    let mut parts = mods_to_string_vec(mods, modifier_format);
    parts.push(key_to_string(key));
    parts
}

fn mods_to_string_vec(mods: Modifiers, modifier_format: ModifierFormat) -> Vec<String> {
    modifier_format
        .iter()
        .filter(|(m, _)| mods.contains(*m))
        .map(|(_, text)| text.to_string())
        .collect()
}

fn key_to_string(key: Code) -> String {
    let key_str = key.to_string();
    KEY_PREFIXES
        .iter()
        .find_map(|prefix| key_str.strip_prefix(prefix))
        .unwrap() // safe since str.strip_prefix("") is no-op
        .to_string()
}

fn parse_part(part: &str, modifier_format: ModifierFormat) -> Modifiers {
    modifier_format
        .iter()
        .find(|(_, text)| *text == part)
        .map(|(m, _)| *m)
        .unwrap_or_else(|| panic!("unknown modifier: {part}"))
}

fn parse_mods(parts: &[&str], modifier_format: ModifierFormat) -> Modifiers {
    parts
        .iter()
        .map(|part| parse_part(part, modifier_format))
        .fold(Modifiers::empty(), |acc, m| acc | m)
}

fn parse_key(string: &str) -> Code {
    KEY_PREFIXES
        .iter()
        .find_map(|prefix| format!("{prefix}{string}").parse::<Code>().ok())
        .unwrap_or_else(|| panic!("unknown key: {string}"))
}
