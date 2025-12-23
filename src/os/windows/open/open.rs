use crate::os::windows::open::win32::focus;
use anyhow::bail;

pub fn open(exe_path: &str) -> anyhow::Result<()> {
    if !focus(exe_path)? {
        bail!("Couldn't find window");
    };
    Ok(())
}
