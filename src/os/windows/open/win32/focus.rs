mod win32 {
    pub use windows::Win32::Foundation::HWND;
    pub use windows::Win32::UI::WindowsAndMessaging::*;
}
use super::windows::{collect_main_windows, find_matching_window};
use anyhow::bail;

pub fn focus(exe_path: &str) -> anyhow::Result<bool> {
    let main_windows = collect_main_windows()?;
    let result = find_matching_window(&main_windows, exe_path)?;
    if let Some(window) = result {
        focus_window(window)?
    }
    Ok(result.is_some())
}

fn focus_window(window: win32::HWND) -> anyhow::Result<()> {
    unsafe {
        if win32::IsIconic(window).as_bool() {
            // undo minimization
            if !win32::ShowWindow(window, win32::SW_RESTORE).as_bool() {
                bail!("syscall 'ShowWindow' failed")
            }
        }
        if !win32::SetForegroundWindow(window).as_bool() {
            bail!("syscall 'SetForegroundWindow' failed")
        }
    }
    Ok(())
}
