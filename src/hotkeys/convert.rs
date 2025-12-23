use super::Hotkey;
use global_hotkey::hotkey::{Code, Modifiers};
use iced::keyboard::Modifiers as IcedModifiers;
use iced::keyboard::key::Physical;
use std::str::FromStr;

pub fn convert_hotkey(modifiers: IcedModifiers, key: Physical) -> Option<Hotkey> {
    Some(Hotkey::new(convert_modifiers(modifiers), convert_key(key)?))
}

fn convert_modifiers(modifiers: IcedModifiers) -> Modifiers {
    let mut mods = Modifiers::empty();
    if modifiers.control() {
        mods |= Modifiers::CONTROL
    }
    if modifiers.logo() {
        mods |= Modifiers::SUPER
    }
    if modifiers.alt() {
        mods |= Modifiers::ALT
    }
    if modifiers.shift() {
        mods |= Modifiers::SHIFT
    }
    mods
}

fn convert_key(key: Physical) -> Option<Code> {
    if let Physical::Code(code) = key {
        let key_str = format!("{:?}", code);
        if key_str.contains("Control")
            || key_str.contains("Super")
            || key_str.contains("Alt")
            || key_str.contains("Shift")
        {
            return None;
        }
        Code::from_str(key_str.as_str()).ok()
    } else {
        None
    }
}
