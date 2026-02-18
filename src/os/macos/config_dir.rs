use std::path::PathBuf;

use crate::os::{ConfigDir, System};

impl ConfigDir for System {
    fn config_dir() -> PathBuf {
        dirs::home_dir()
            .expect("could not determine home directory")
            .join(".config")
    }
}
