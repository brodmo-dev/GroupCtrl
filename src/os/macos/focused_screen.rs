use objc2_app_kit::NSScreen;
use objc2_foundation::MainThreadMarker;

use super::System;
use crate::os::traits::FocusedScreen;

impl FocusedScreen for System {
    fn focused_screen() -> Option<(f64, f64, f64, f64)> {
        let mtm = MainThreadMarker::new()?;
        let frame = NSScreen::mainScreen(mtm)?.visibleFrame(); // main = has focused window
        let primary_height = NSScreen::screens(mtm).objectAtIndex(0).frame().size.height;
        // convert to tao coordinates -- primary monitor is the reference
        let y = primary_height - (frame.origin.y + frame.size.height);
        Some((frame.origin.x, y, frame.size.width, frame.size.height))
    }
}
