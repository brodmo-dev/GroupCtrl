use std::path::PathBuf;

use crate::os::{ConfigDir, System};

const APP_DIR_NAME: &str = "groupctrl";

pub fn config_dir() -> PathBuf {
    System::config_dir().join(APP_DIR_NAME)
}

pub fn icons_dir() -> PathBuf {
    dirs::data_local_dir()
        .expect("could not determine data local directory")
        .join(APP_DIR_NAME)
        .join("icons")
}
