use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::models::Identifiable;
use crate::os::AppMetadata;
use crate::util::capitalize;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(into = "String", from = "String")]
pub struct App {
    pub(super) exe_path: String,
    name: String,
    icon_path: Option<PathBuf>,
}

impl AppMetadata for App {
    fn name(&self) -> &str {
        &self.name
    }

    fn icon_path(&self) -> Option<&Path> {
        self.icon_path.as_deref()
    }
}

impl App {
    fn heuristic_name(exe_path: &str) -> String {
        let exe_name = exe_path.split('\\').last().unwrap_or(exe_path);
        let name = exe_name.strip_suffix(".exe").unwrap_or(exe_name);
        capitalize(name)
    }
}

impl Identifiable<String> for App {
    fn id(&self) -> String {
        self.exe_path.clone()
    }
}

impl From<App> for String {
    fn from(app: App) -> Self {
        app.exe_path
    }
}

impl From<String> for App {
    fn from(exe_path: String) -> Self {
        let name = Self::heuristic_name(&exe_path);
        let icon_path = None;
        Self {
            exe_path,
            name,
            icon_path,
        }
    }
}

impl Display for App {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Hash for App {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.exe_path.hash(state);
    }
}

impl PartialEq for App {
    fn eq(&self, other: &Self) -> bool {
        self.exe_path == other.exe_path
    }
}

impl Eq for App {}
