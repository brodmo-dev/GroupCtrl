use std::sync::Once;

use dioxus::desktop::tao::platform::macos::WindowExtMacOS;
use objc2::ffi::object_setClass;
use objc2::runtime::{AnyClass, AnyObject, Bool, ClassBuilder, Sel};
use objc2::sel;
use objc2_app_kit::{NSFloatingWindowLevel, NSWindow, NSWindowStyleMask};

use super::System;
use crate::os::traits::LauncherWindow;

impl LauncherWindow for System {
    fn configure_launcher_window(window: &dioxus::desktop::DesktopContext) {
        let ns_window_ptr = window.window.ns_window();
        let panel_class = launcher_panel_class();
        unsafe { object_setClass(ns_window_ptr as *mut _, panel_class as *const _ as *mut _) };

        let ns_window = unsafe { &*(ns_window_ptr as *const NSWindow) };
        let mask = ns_window.styleMask();
        ns_window.setStyleMask(NSWindowStyleMask(
            mask.0 | NSWindowStyleMask::NonactivatingPanel.0,
        ));
        ns_window.setLevel(NSFloatingWindowLevel);
    }

    fn show_launcher_window(window: &dioxus::desktop::DesktopContext) {
        let ns_window_ptr = window.window.ns_window();
        let ns_view_ptr = window.window.ns_view();

        let ns_window = unsafe { &*(ns_window_ptr as *const NSWindow) };
        ns_window.makeKeyAndOrderFront(None);

        // Make the WKWebView (ns_view) the first responder so keyboard events reach the DOM
        let ns_view = unsafe { &*(ns_view_ptr as *const objc2_app_kit::NSResponder) };
        ns_window.makeFirstResponder(Some(ns_view));
    }

    fn hide_launcher_window(window: &dioxus::desktop::DesktopContext) {
        let ns_window_ptr = window.window.ns_window();
        let ns_window = unsafe { &*(ns_window_ptr as *const NSWindow) };
        ns_window.orderOut(None);
    }
}

fn launcher_panel_class() -> &'static AnyClass {
    static REGISTER: Once = Once::new();
    REGISTER.call_once(|| {
        let panel_class = AnyClass::get(c"NSPanel").unwrap();
        let mut builder = ClassBuilder::new(c"LauncherPanel", panel_class).unwrap();

        unsafe {
            builder.add_method(
                sel!(canBecomeKeyWindow),
                can_become_key_window as extern "C-unwind" fn(_, _) -> Bool,
            );
        }

        builder.register();
    });

    AnyClass::get(c"LauncherPanel").unwrap()
}

extern "C-unwind" fn can_become_key_window(_this: *mut AnyObject, _sel: Sel) -> Bool {
    Bool::YES
}
