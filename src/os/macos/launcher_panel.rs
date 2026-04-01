use std::ffi::c_void;
use std::sync::Once;

use objc2::ffi::object_setClass;
use objc2::runtime::{AnyClass, AnyObject, ClassBuilder, Sel};
use objc2::{msg_send, sel};
use objc2_app_kit::{NSApplication, NSFloatingWindowLevel, NSWindow, NSWindowStyleMask};
use objc2_foundation::MainThreadMarker;

/// Register a one-off "LauncherPanel" ObjC class that subclasses NSPanel
/// and overrides `becomeKey` to call `NSApp.activate()` (per the article's
/// approach for non-activating panels that still receive keyboard input).
fn launcher_panel_class() -> &'static AnyClass {
    static REGISTER: Once = Once::new();
    REGISTER.call_once(|| {
        let panel_class = AnyClass::get(c"NSPanel").unwrap();
        let mut builder = ClassBuilder::new(c"LauncherPanel", panel_class).unwrap();

        unsafe {
            builder.add_method(sel!(becomeKey), become_key as extern "C-unwind" fn(_, _));
            builder.add_method(sel!(close), close as extern "C-unwind" fn(_, _));
            builder.add_method(sel!(resignKey), resign_key as extern "C-unwind" fn(_, _));
        }

        builder.register();
    });

    AnyClass::get(c"LauncherPanel").unwrap()
}

extern "C-unwind" fn become_key(this: *mut AnyObject, _sel: Sel) {
    let superclass = AnyClass::get(c"NSPanel").unwrap();
    unsafe {
        let this = &*this;
        let _: () = msg_send![super(this, superclass), becomeKey];
        if let Some(mtm) = MainThreadMarker::new() {
            NSApplication::sharedApplication(mtm).activate();
        }
    }
}

extern "C-unwind" fn close(this: *mut AnyObject, _sel: Sel) {
    let superclass = AnyClass::get(c"NSPanel").unwrap();
    unsafe {
        let this = &*this;
        let _: () = msg_send![super(this, superclass), close];
        if let Some(mtm) = MainThreadMarker::new() {
            let app = NSApplication::sharedApplication(mtm);
            let _: () = msg_send![&app, hide: this];
        }
    }
}

extern "C-unwind" fn resign_key(this: *mut AnyObject, _sel: Sel) {
    unsafe {
        let _: () = msg_send![&*this, close];
    }
}

/// Turns a tao NSWindow into our LauncherPanel (NSPanel subclass) with
/// `.nonactivatingPanel` style mask.
pub fn configure_as_panel(ns_window_ptr: *mut c_void) {
    let panel_class = launcher_panel_class();
    unsafe { object_setClass(ns_window_ptr as *mut _, panel_class as *const _ as *mut _) };

    let ns_window = unsafe { &*(ns_window_ptr as *const NSWindow) };
    let mask = ns_window.styleMask();
    ns_window.setStyleMask(NSWindowStyleMask(
        mask.0 | NSWindowStyleMask::NonactivatingPanel.0,
    ));
    ns_window.setLevel(NSFloatingWindowLevel);
}

pub fn show_panel(ns_window_ptr: *mut c_void, ns_view_ptr: *mut c_void) {
    let ns_window = unsafe { &*(ns_window_ptr as *const NSWindow) };
    ns_window.makeKeyAndOrderFront(None);
    // Make the WKWebView first responder so keyboard events reach the DOM
    let ns_view = unsafe { &*(ns_view_ptr as *const objc2_app_kit::NSResponder) };
    ns_window.makeFirstResponder(Some(ns_view));
}

pub fn hide_panel(ns_window_ptr: *mut c_void) {
    let ns_window = unsafe { &*(ns_window_ptr as *const NSWindow) };
    ns_window.orderOut(None);
}
