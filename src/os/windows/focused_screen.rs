use super::System;
use crate::os::traits::FocusedScreen;

impl FocusedScreen for System {
    fn focused_screen() -> Option<(f64, f64, f64, f64)> {
        None
    }
}
