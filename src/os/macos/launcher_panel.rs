use std::ffi::CStr;
use std::sync::LazyLock;

use dioxus::desktop::tao::platform::macos::WindowExtMacOS;
use objc2::ffi::object_setClass;
use objc2::runtime::{AnyClass, AnyObject, Bool, ClassBuilder, Sel};
use objc2::sel;
use objc2_app_kit::{NSFloatingWindowLevel, NSPanel, NSResponder, NSWindowStyleMask};

use super::System;
use crate::os::traits::LauncherWindow;

const LAUNCHER_PANEL_NAME: &CStr = c"LauncherPanel";
static LAUNCHER_PANEL_CLASS: LazyLock<&'static AnyClass> =
    LazyLock::new(|| register_class().expect("LauncherPanel class registration failed"));

fn register_class() -> Option<&'static AnyClass> {
    let mut builder = ClassBuilder::new(LAUNCHER_PANEL_NAME, AnyClass::get(c"NSPanel")?)?;
    unsafe {
        builder.add_method(
            sel!(canBecomeKeyWindow),
            can_become_key_window as extern "C-unwind" fn(_, _) -> Bool,
        );
    }
    builder.register();
    AnyClass::get(LAUNCHER_PANEL_NAME)
}

extern "C-unwind" fn can_become_key_window(_this: *mut AnyObject, _sel: Sel) -> Bool {
    Bool::YES
}

impl LauncherWindow for System {
    fn configure_launcher_window(window: &dioxus::desktop::DesktopContext) {
        let ns_window_ptr = window.window.ns_window();
        // Isa-swizzle to our custom class (registers on first access via LazyLock)
        unsafe {
            object_setClass(
                ns_window_ptr as *mut _,
                *LAUNCHER_PANEL_CLASS as *const _ as *mut _,
            )
        };
        let panel = unsafe { &*(ns_window_ptr as *const NSPanel) };
        let mask = panel.styleMask().0 | NSWindowStyleMask::NonactivatingPanel.0;
        panel.setStyleMask(NSWindowStyleMask(mask));
        panel.setLevel(NSFloatingWindowLevel);
    }

    fn show_launcher_window(window: &dioxus::desktop::DesktopContext) {
        let panel = unsafe { &*(window.window.ns_window() as *const NSPanel) };
        let view = unsafe { &*(window.window.ns_view() as *const NSResponder) };
        panel.makeKeyAndOrderFront(None);
        panel.makeFirstResponder(Some(view));
    }

    fn hide_launcher_window(window: &dioxus::desktop::DesktopContext) {
        let panel = unsafe { &*(window.window.ns_window() as *const NSPanel) };
        panel.orderOut(None);
    }
}
