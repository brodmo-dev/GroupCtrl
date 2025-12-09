use crate::app::App;
use crate::open::Open;
use global_hotkey::{
    GlobalHotKeyEvent, GlobalHotKeyManager,
    hotkey::{Code, HotKey, Modifiers},
};
use std::thread;

pub struct HotkeyManager {
    _global_manager: GlobalHotKeyManager,
}

impl HotkeyManager {
    fn listen_for_hotkeys() {
        loop {
            if let Ok(event) = GlobalHotKeyEvent::receiver().recv()
                && event.state == global_hotkey::HotKeyState::Pressed
            {
                App::new("com.apple.finder").open().unwrap();
            }
        }
    }

    pub fn new() -> Self {
        let manager = GlobalHotKeyManager::new().unwrap();
        // TODO these are HARDWARE keys!
        let hotkey = HotKey::new(Some(Modifiers::SUPER | Modifiers::SHIFT), Code::KeyF);
        manager.register(hotkey).unwrap();
        thread::spawn(Self::listen_for_hotkeys);
        Self {
            // need to keep it alive
            _global_manager: manager,
        }
    }
}
