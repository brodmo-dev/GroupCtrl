use crate::action::Action;
use anyhow::Result;
use crossbeam::channel;
use global_hotkey::hotkey::HotKey;
use global_hotkey::{GlobalHotKeyEvent, GlobalHotKeyManager};
use std::collections::HashMap;
use std::thread;

// We need to store u32 because that's all we get from the keypress event
pub type HotkeyBinding = (u32, Action);

pub fn listen_for_hotkeys(binding_receiver: channel::Receiver<HotkeyBinding>) {
    let mut hotkey_actions = HashMap::new();
    let hotkey_receiver = GlobalHotKeyEvent::receiver();
    loop {
        crossbeam::select! {
            recv(binding_receiver) -> binding => {
                if let Ok((hotkey, action)) = binding {
                    hotkey_actions.insert(hotkey, action);
                }
            }
            recv(hotkey_receiver) -> optional_event => {
                if let Ok(event) = optional_event
                    && event.state == global_hotkey::HotKeyState::Pressed
                    && let Some(action) = hotkey_actions.get(&event.id)
                {
                    action.execute()
                }
            }
        }
    }
}

pub struct HotkeyManager {
    global_manager: GlobalHotKeyManager,
    action_sender: channel::Sender<HotkeyBinding>,
}

impl HotkeyManager {
    pub fn new() -> Self {
        let manager = GlobalHotKeyManager::new().unwrap();
        let (tx, rx) = channel::unbounded();
        thread::spawn(move || listen_for_hotkeys(rx));
        Self {
            global_manager: manager,
            action_sender: tx,
        }
    }

    pub fn bind_hotkey(&self, hotkey: HotKey, action: Action) -> Result<()> {
        self.global_manager.register(hotkey)?;
        self.action_sender.send((hotkey.id(), action))?;
        Ok(())
    }
}
