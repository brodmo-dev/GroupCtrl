mod action;
mod app;
mod hotkeys;
mod open;
mod util;

use crate::hotkeys::{HotkeyManager, HotkeyPicker, PickerMessage};
use anyhow::Result;
use iced::Element;
use simplelog::*;
use std::fs;
use std::fs::File;

#[derive(Default)]
struct GroupCtrl {
    hotkey_manager: HotkeyManager,
    hotkey_picker: HotkeyPicker,
}

#[derive(Clone, Debug)]
enum Message {
    Picker(PickerMessage),
}

impl GroupCtrl {
    fn update(&mut self, message: Message) {
        match message {
            Message::Picker(picker_msg) => {
                self.hotkey_picker
                    .update(picker_msg, &mut self.hotkey_manager);
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        Element::from(self.hotkey_picker.view()).map(Message::Picker)
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        self.hotkey_picker.subscription().map(Message::Picker)
    }
}

fn setup_logging() -> Result<()> {
    fs::create_dir_all("logs")?;
    let log_file = File::create("logs/app.log")?;
    let config = ConfigBuilder::new().build();
    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Info,
            config.clone(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(LevelFilter::Debug, config, log_file),
    ])?;
    Ok(())
}

fn main() -> iced::Result {
    setup_logging().expect("Logging setup failed");
    iced::application(GroupCtrl::default, GroupCtrl::update, GroupCtrl::view)
        .subscription(GroupCtrl::subscription)
        .run()
}
