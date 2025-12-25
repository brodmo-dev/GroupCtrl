use crate::os::{App, Openable};

mod open;
mod win32;

impl Openable for App {
    fn open(&self) -> anyhow::Result<()> {
        open::open(&self.exe_path)
    }
}
