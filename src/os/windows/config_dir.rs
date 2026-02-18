use std::path::PathBuf;

use crate::os::{ConfigDir, System};

impl ConfigDir for System {
    fn config_dir() -> PathBuf {
        dirs::config_dir().expect("could not determine config directory")
    }
}
