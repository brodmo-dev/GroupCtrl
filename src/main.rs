mod action;
mod app;
mod hotkeys;
mod open;

use crate::action::Action;
use crate::app::App;
use crate::hotkeys::HotkeyManager;
use anyhow::Error;
use eframe::egui;
use eframe::egui::Button;
use global_hotkey::hotkey::{Code, HotKey, Modifiers};

struct GroupCtrl {
    hotkey_manager: HotkeyManager,
    error: Option<Error>,
}

impl GroupCtrl {
    fn new() -> Self {
        Self {
            hotkey_manager: HotkeyManager::new(),
            error: None,
        }
    }
}

impl eframe::App for GroupCtrl {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let button = Button::new("Register Finder hotkey");
            if ui.add(button).clicked() {
                let hotkey = HotKey::new(Some(Modifiers::SUPER | Modifiers::SHIFT), Code::KeyF);
                let action = Action::OpenApp(App::new("com.apple.finder"));
                self.error = self.hotkey_manager.bind_hotkey(hotkey, action).err()
            }
            if let Some(error) = &self.error {
                ui.colored_label(egui::Color32::RED, format!("Error: {}", error));
            }
        });
    }
}

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "GroupCtrl",
        eframe::NativeOptions::default(),
        Box::new(|_| Ok(Box::new(GroupCtrl::new()))),
    )
}
