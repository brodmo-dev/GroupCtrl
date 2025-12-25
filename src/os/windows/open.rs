use crate::os::App;
use crate::os::prelude::Openable;

mod open;
mod win32;

impl Openable for App {
    fn open(&self) -> anyhow::Result<()> {
        open::open(&self.exe_path)
    }
}
