use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use anyhow::anyhow;
use dioxus::desktop::{ShortcutHandle, window};
use global_hotkey::HotKeyState::Pressed;

use crate::models::{Action, Hotkey};

pub trait HotkeyBinder {
    fn bind_hotkey(&mut self, hotkey: Hotkey, action: &Action) -> anyhow::Result<()>;
    fn unbind_hotkey(&mut self, hotkey: Hotkey);
}

pub struct DioxusBinder {
    #[allow(clippy::type_complexity)] // Gotta fix this anyhow
    recording_callback: Arc<Mutex<Option<Arc<dyn Fn(Hotkey) + Send + Sync>>>>,
    handles: HashMap<Hotkey, ShortcutHandle>,
}

impl DioxusBinder {
    pub fn new() -> Self {
        Self {
            recording_callback: Arc::new(Mutex::new(None)),
            handles: HashMap::new(),
        }
    }

    pub fn set_recording_callback(&mut self, callback: Arc<dyn Fn(Hotkey) + Send + Sync>) {
        let mut cb = self.recording_callback.lock().unwrap();
        *cb = Some(callback);
    }

    pub fn clear_recording_callback(&mut self) {
        let mut cb = self.recording_callback.lock().unwrap();
        *cb = None;
    }
}

impl HotkeyBinder for DioxusBinder {
    fn bind_hotkey(&mut self, hotkey: Hotkey, action: &Action) -> anyhow::Result<()> {
        let my_action = action.clone();
        let recording_callback = self.recording_callback.clone();
        let callback = move |state| {
            if state == Pressed {
                let cb = recording_callback.lock().unwrap();
                if let Some(picker_callback) = cb.as_ref() {
                    picker_callback(hotkey);
                } else {
                    let _ = my_action.execute();
                }
            }
        };
        let handle = window()
            .create_shortcut(hotkey.0, callback)
            // manual error mapping because this error doesn't implement Display
            .map_err(|e| anyhow!("Failed to create shortcut: {:?}", e))?;
        self.handles.insert(hotkey, handle);
        Ok(())
    }

    fn unbind_hotkey(&mut self, hotkey: Hotkey) {
        let handle = self.handles.remove(&hotkey).unwrap();
        window().remove_shortcut(handle);
    }
}

#[cfg(test)]
pub mod tests {
    use std::sync::Mutex;

    use super::*;

    #[derive(Debug, PartialEq, Clone)]
    pub enum MockEvent {
        Register(Hotkey, Action),
        Unregister(Hotkey),
    }

    pub struct MockBinder {
        pub events: Arc<Mutex<Vec<MockEvent>>>,
    }

    impl HotkeyBinder for MockBinder {
        fn bind_hotkey(&mut self, hotkey: Hotkey, action: &Action) -> anyhow::Result<()> {
            let mut events = self.events.lock().unwrap();
            events.push(MockEvent::Register(hotkey, action.clone()));
            Ok(())
        }

        fn unbind_hotkey(&mut self, hotkey: Hotkey) {
            let mut events = self.events.lock().unwrap();
            events.push(MockEvent::Unregister(hotkey));
        }
    }
}
