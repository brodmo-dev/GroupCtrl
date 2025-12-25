use std::fmt::{Display, Formatter};

use super::Hotkey;
use crate::os::App;

#[derive(Debug)]
#[allow(unused)]
struct Group {
    name: String,
    hotkey: Option<Hotkey>,
    members: Vec<App>,
}

impl Display for Group {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
