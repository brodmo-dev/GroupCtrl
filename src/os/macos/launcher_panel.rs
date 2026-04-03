use std::ffi::CStr;
use std::sync::LazyLock;

use dioxus::desktop::DesktopContext;
use dioxus::desktop::tao::platform::macos::WindowExtMacOS;
use objc2::ffi::object_setClass;
use objc2::runtime::{AnyClass, AnyObject, Bool, ClassBuilder, Sel};
use objc2::sel;
use objc2_app_kit::{NSModalPanelWindowLevel, NSPanel, NSWindowStyleMask};

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

fn ns_panel(window: &DesktopContext) -> &NSPanel {
    unsafe { &*(window.window.ns_window() as *const NSPanel) }
}

impl LauncherWindow for System {
    fn configure_launcher_window(window: &DesktopContext) {
        // Isa-swizzle to our custom class (registers on first access via LazyLock)
        unsafe {
            object_setClass(
                window.window.ns_window() as *mut _,
                *LAUNCHER_PANEL_CLASS as *const _ as *mut _,
            )
        };
        let panel = ns_panel(window);
        let mask = panel.styleMask().0 | NSWindowStyleMask::NonactivatingPanel.0;
        panel.setStyleMask(NSWindowStyleMask(mask));
        panel.setLevel(NSModalPanelWindowLevel);
    }

    fn show_launcher_window(window: &DesktopContext) {
        ns_panel(window).makeKeyAndOrderFront(None);
    }

    fn hide_launcher_window(window: &DesktopContext) {
        ns_panel(window).orderOut(None);
    }
}
