mod binder;
mod error;
mod sender;

use binder::{DioxusBinder, HotkeyBinder};
use dioxus::hooks::UnboundedSender;
pub use error::HotkeyBindError;
pub use sender::SharedSender;

use crate::models::{Action, Hotkey};
use crate::services::config_reader::ConfigReader;

pub struct HotkeyService<B: HotkeyBinder = DioxusBinder> {
    binder: B,
    config_reader: ConfigReader,
}

impl HotkeyService<DioxusBinder> {
    pub fn new(
        config_reader: ConfigReader,
        record_registered_sender: SharedSender<Hotkey>,
        action_sender: UnboundedSender<Action>,
    ) -> Self {
        Self {
            config_reader,
            binder: DioxusBinder::new(record_registered_sender, action_sender),
        }
    }
}

impl<B: HotkeyBinder> HotkeyService<B> {
    fn find_conflict(&self, hotkey: Hotkey) -> Option<Action> {
        self.config_reader
            .read()
            .bindings()
            .into_iter()
            .find_map(|(hk, a)| (hk == Some(hotkey)).then_some(a))
    }

    pub fn bind_hotkey(
        &mut self,
        hotkey: Option<Hotkey>,
        existing_hotkey: Option<Hotkey>,
        action: Action,
    ) -> Result<(), HotkeyBindError> {
        if hotkey == existing_hotkey {
            return Ok(());
        }
        if let Some(hk) = hotkey
            && let Some(conflict) = self.find_conflict(hk)
        {
            return Err(HotkeyBindError::Conflict {
                hotkey: hk,
                conflict: conflict.describe(&self.config_reader.read()),
            });
        }

        self.unbind_hotkey(existing_hotkey);
        if let Some(hk) = hotkey {
            self.binder.bind_hotkey(hk, &action)?
        }
        Ok(())
    }

    pub fn unbind_hotkey(&mut self, hotkey: Option<Hotkey>) {
        if let Some(hk) = hotkey {
            self.binder.unbind_hotkey(hk);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex, RwLock};

    use global_hotkey::hotkey::{Code, Modifiers};

    use super::binder::tests::MockBinder;
    use super::binder::tests::MockEvent::*;
    use super::*;
    use crate::models::Config;
    use crate::services::ConfigService;
    use crate::services::hotkey_service::binder::tests::MockEvent;

    impl HotkeyService<MockBinder> {
        fn new_mock(config_reader: ConfigReader, binder: MockBinder) -> Self {
            Self {
                binder,
                config_reader,
            }
        }
    }

    fn setup_service() -> (
        Arc<RwLock<Config>>,
        HotkeyService<MockBinder>,
        Arc<Mutex<Vec<MockEvent>>>,
    ) {
        let config = Arc::new(RwLock::new(Config::default()));
        // TODO should be tx/rx instead of arc mutex
        let events = Arc::new(Mutex::new(Vec::new()));
        let binder = MockBinder {
            events: events.clone(),
        };
        let config_reader = ConfigReader::new(config.clone());
        let service = HotkeyService::new_mock(config_reader, binder);
        (config, service, events)
    }

    fn setup_group(config: Arc<RwLock<Config>>, name: &str, hotkey: Option<Hotkey>) -> Action {
        let group_id = config.write().unwrap().add_group(name.to_string());
        let _ = config.write().unwrap().set_hotkey(group_id, hotkey);
        Action::OpenGroup { group_id }
    }

    #[test]
    fn bind_hotkey_new() {
        // Arrange
        let (config, mut service, events) = setup_service();
        let action = setup_group(config, "Test", None);
        let hotkey = Hotkey::new(Modifiers::SUPER | Modifiers::SHIFT, Code::KeyF);

        // Act
        let result = service.bind_hotkey(Some(hotkey), None, action.clone());

        // Assert
        assert_eq!(result, Ok(()));
        assert_eq!(*events.lock().unwrap(), vec![Register(hotkey, action)]);
    }

    #[test]
    fn bind_hotkey_repeat_none() {
        // Arrange
        let (config, mut service, events) = setup_service();
        let action = setup_group(config, "Test", None);

        // Act
        let result = service.bind_hotkey(None, None, action.clone());

        // Assert
        assert_eq!(result, Ok(()));
        assert_eq!(*events.lock().unwrap(), vec![]);
    }

    #[test]
    fn bind_hotkey_repeat_some() {
        // Arrange
        let (config, mut service, events) = setup_service();
        let action = setup_group(config, "Test", None);
        let hotkey = Hotkey::new(Modifiers::SUPER | Modifiers::SHIFT, Code::KeyF);

        // Act
        let result = service.bind_hotkey(Some(hotkey), Some(hotkey), action.clone());

        // Assert
        assert_eq!(result, Ok(()));
        assert_eq!(*events.lock().unwrap(), vec![]);
    }

    #[test]
    fn bind_hotkey_change() {
        // Arrange
        let (config, mut service, events) = setup_service();
        let old_hotkey = Hotkey::new(Modifiers::SUPER | Modifiers::SHIFT, Code::KeyF);
        let new_hotkey = Hotkey::new(Modifiers::SUPER | Modifiers::SHIFT, Code::KeyG);
        let action = setup_group(config, "Test", Some(old_hotkey));

        // Act
        let result = service.bind_hotkey(Some(new_hotkey), Some(old_hotkey), action.clone());

        // Assert
        assert_eq!(result, Ok(()));
        assert_eq!(
            *events.lock().unwrap(),
            vec![Unregister(old_hotkey), Register(new_hotkey, action)]
        );
    }

    #[test]
    fn bind_hotkey_conflict() {
        // Arrange
        let (config, mut service, events) = setup_service();
        let hotkey = Hotkey::new(Modifiers::SUPER | Modifiers::SHIFT, Code::KeyF);
        setup_group(config.clone(), "Fst", Some(hotkey));
        let new_action = setup_group(config, "Snd", None);

        // Act
        let result = service.bind_hotkey(Some(hotkey), None, new_action);

        // Assert
        assert_eq!(
            result,
            Err(HotkeyBindError::Conflict {
                hotkey,
                conflict: "open group 'Fst'".to_string()
            })
        );
        assert_eq!(*events.lock().unwrap(), vec![]);
    }

    #[test]
    fn unbind_hotkey() {
        // Arrange
        let (_config, mut service, events) = setup_service();
        let hotkey = Hotkey::new(Modifiers::SUPER | Modifiers::SHIFT, Code::KeyF);

        // Act
        service.unbind_hotkey(Some(hotkey));

        // Assert
        assert_eq!(*events.lock().unwrap(), vec![Unregister(hotkey)]);
    }
}
