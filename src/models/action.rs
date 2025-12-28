use std::fmt::{Display, Formatter};

use uuid::Uuid;

use crate::models::Hotkey;
use crate::models::group::Group;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Action {
    OpenGroup {
        group_id: Uuid,
    },
    #[cfg(test)]
    Mock(&'static str),
}

impl Display for Action {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Action::OpenGroup { group_id } => format!("Open group {group_id}"),
            #[cfg(test)]
            Action::Mock(str) => format!("Mock {str}"),
        };
        write!(f, "{msg}")
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
