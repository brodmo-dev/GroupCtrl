mod app;
mod hotkeys;
mod open;

use crate::hotkeys::HotkeyManager;
use eframe::egui;

struct GroupCtrl {
    _hotkey_manager: HotkeyManager,
}

impl GroupCtrl {
    fn new() -> Self {
        Self {
            _hotkey_manager: HotkeyManager::new(),
        }
    }
}

impl eframe::App for GroupCtrl {
    fn update(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // draw UI
    }
}

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "GroupCtrl",
        eframe::NativeOptions::default(),
        Box::new(|_| Ok(Box::new(GroupCtrl::new()))),
    )
}
