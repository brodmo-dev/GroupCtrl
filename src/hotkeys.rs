use crate::action::Action;
use anyhow::Result;
use global_hotkey::hotkey::HotKey;
use global_hotkey::{GlobalHotKeyEvent, GlobalHotKeyManager};
use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;

pub type HotkeyAction = Box<dyn Action + 'static>;

pub struct HotkeyListener {
    hotkey_actions: HashMap<u32, HotkeyAction>,
    action_receiver: mpsc::Receiver<(u32, HotkeyAction)>,
}

impl HotkeyListener {
    pub fn new(action_receiver: mpsc::Receiver<(u32, HotkeyAction)>) -> Self {
        HotkeyListener {
            hotkey_actions: HashMap::new(),
            action_receiver,
        }
    }

    pub fn listen_for_hotkeys(&mut self) {
        self.update_hotkey_actions();
        loop {
            if let Ok(event) = GlobalHotKeyEvent::receiver().recv()
                && event.state == global_hotkey::HotKeyState::Pressed
            {
                self.update_hotkey_actions();
                if let Some(action) = self.hotkey_actions.get(&event.id) {
                    action.execute()
                }
            }
        }
    }

    fn update_hotkey_actions(&mut self) {
        while let Ok((id, callback)) = self.action_receiver.try_recv() {
            self.hotkey_actions.insert(id, callback);
        }
    }
}

pub struct HotkeyManager {
    global_manager: GlobalHotKeyManager,
    action_sender: mpsc::Sender<(u32, HotkeyAction)>,
}

impl HotkeyManager {
    pub fn new() -> Self {
        let manager = GlobalHotKeyManager::new().unwrap();
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || HotkeyListener::new(rx).listen_for_hotkeys());
        Self {
            global_manager: manager,
            action_sender: tx,
        }
    }

    pub fn register_hotkey<T: Action + 'static>(&self, hotkey: HotKey, action: T) -> Result<()> {
        self.global_manager.register(hotkey)?;
        self.action_sender.send((hotkey.id(), Box::new(action)))?;
        Ok(())
    }
}
