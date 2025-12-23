use crate::os::App;
use crate::os::prelude::Openable;
use crate::os::windows::open::open::open;

mod open;
mod win32;

impl Openable for App {
    fn open(&self) -> anyhow::Result<()> {
        open(&self.exe_path)
    }
}
