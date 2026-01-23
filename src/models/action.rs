use uuid::Uuid;

use crate::models::group::Group;
use crate::models::{Config, Hotkey, Identifiable};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Action {
    OpenGroup { group_id: Uuid },
}

impl Action {
    pub fn describe(&self, config: &Config) -> String {
        match self {
            Action::OpenGroup { group_id } => {
                format!("Open Group {}", config.group(*group_id).unwrap().name)
            }
        }
    }
}

pub trait Bindable {
    fn binding(&self) -> (Option<Hotkey>, Action);
}

impl Bindable for Group {
    fn binding(&self) -> (Option<Hotkey>, Action) {
        let action = Action::OpenGroup {
            group_id: self.id(),
        };
        (self.hotkey, action)
    }
}
