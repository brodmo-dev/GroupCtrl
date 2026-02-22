use objc2::MainThreadMarker;
use objc2_app_kit::{NSApplication, NSApplicationActivationPolicy};

use super::System;
use crate::os::WindowConfiguration;

impl WindowConfiguration for System {
    fn configure_window() {
        unsafe {
            let mtm = MainThreadMarker::new_unchecked();
            NSApplication::sharedApplication(mtm)
                .setActivationPolicy(NSApplicationActivationPolicy::Accessory);
        }
    }
}
