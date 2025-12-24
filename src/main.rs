mod components;
mod models;
mod os;
mod services;
mod util;

use std::fs;

use iced::widget::{button, row, text};
use iced::{Element, Task};
use simplelog::*;

use crate::components::hotkey_picker::{HotkeyPicker, Message as PickerMessage};
use crate::models::action::Action;
use crate::os::prelude::AppPickerTrait;
use crate::os::{App, AppPicker};
use crate::services::hotkey::HotkeyService;

#[derive(Default)]
struct GroupCtrl {
    hotkey_service: HotkeyService,
    hotkey_picker: HotkeyPicker,
    selected_app: Option<App>,
}

#[derive(Clone, Debug)]
enum Message {
    Picker(PickerMessage),
    AppPicked(Option<App>),
    PickApp,
}

impl GroupCtrl {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Picker(picker_message) => {
                if let Some(app) = &self.selected_app {
                    let action = Action::OpenApp(app.clone());
                    self.hotkey_picker
                        .update(picker_message, &mut self.hotkey_service, action);
                }
                Task::none()
            }
            Message::PickApp => Task::perform(AppPicker::pick_app(), |result| {
                Message::AppPicked(result.ok().flatten()) // Discard error
            }),
            Message::AppPicked(app_option) => {
                self.selected_app = app_option;
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        row![
            Element::from(self.hotkey_picker.view()).map(Message::Picker),
            if let Some(app) = &self.selected_app {
                text(&app.bundle_id)
            } else {
                text("No app selected")
            },
            button("Pick App").on_press(Message::PickApp)
        ]
        .into()
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        self.hotkey_picker.subscription().map(Message::Picker)
    }
}

fn setup_logging() -> anyhow::Result<()> {
    fs::create_dir_all("logs")?;
    let log_file = fs::File::create("logs/app.log")?;
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

    // Make panics crash loudly during development
    std::panic::set_hook(Box::new(|panic_info| {
        eprintln!("PANIC: {}", panic_info);
        std::process::exit(1);
    }));

    iced::application(GroupCtrl::default, GroupCtrl::update, GroupCtrl::view)
        .subscription(GroupCtrl::subscription)
        .run()
}
