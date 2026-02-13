use std::ptr::NonNull;
use std::sync::mpsc::{self, Receiver};

use block2::RcBlock;
use objc2_app_kit::{
    NSRunningApplication, NSWorkspace, NSWorkspaceApplicationKey,
    NSWorkspaceDidActivateApplicationNotification,
};
use objc2_foundation::NSNotification;

use crate::os::{AppObserver, System};

impl AppObserver for System {
    fn observe_app_activations() -> Receiver<String> {
        let (tx, rx) = mpsc::channel();
        let block = RcBlock::new(move |notification: NonNull<NSNotification>| {
            if let Some(bundle_id) = extract_bundle_id(notification) {
                let _ = tx.send(bundle_id);
            }
        });
        unsafe {
            NSWorkspace::sharedWorkspace()
                .notificationCenter()
                .addObserverForName_object_queue_usingBlock(
                    Some(NSWorkspaceDidActivateApplicationNotification),
                    None,
                    None,
                    &block,
                );
        }
        rx
    }
}

fn extract_bundle_id(notification: NonNull<NSNotification>) -> Option<String> {
    let info = unsafe { notification.as_ref() }.userInfo()?;
    let obj = info.objectForKey(unsafe { NSWorkspaceApplicationKey }.as_ref())?;
    let app = obj.downcast::<NSRunningApplication>().ok()?;
    Some(app.bundleIdentifier()?.to_string())
}
