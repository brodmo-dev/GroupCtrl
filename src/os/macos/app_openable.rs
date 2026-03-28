use anyhow::{Context, anyhow};
use block2::RcBlock;
use futures::StreamExt;
use log::info;
use objc2_app_kit::{NSWorkspace, NSWorkspaceOpenConfiguration};
use objc2_foundation::{NSError, NSString, NSURL};

use super::app::App;
use super::app_metadata::resolve_app_path;
use crate::os::Openable;

impl Openable for App {
    async fn open(id: &str) -> anyhow::Result<()> {
        info!("opening app '{id}'");
        let path =
            resolve_app_path(id).context(format!("could not find app with bundle id '{id}'"))?;
        let app_url = NSURL::fileURLWithPath(&NSString::from_str(&path));
        let (tx, mut rx) = futures::channel::mpsc::unbounded();
        let handler_app_url = app_url.clone();
        let handler = RcBlock::new(move |_app, error: *mut NSError| {
            let _ = tx.unbounded_send(if error.is_null() {
                Ok(())
            } else {
                Err(anyhow!(
                    "could not open app at path '{}': {}",
                    handler_app_url.path().unwrap(),
                    unsafe { &*error }
                ))
            });
        });
        NSWorkspace::sharedWorkspace().openApplicationAtURL_configuration_completionHandler(
            &app_url,
            &NSWorkspaceOpenConfiguration::configuration(),
            Some(&handler),
        );
        rx.next()
            .await
            .context("openApplicationAtUrl completion handler was discarded")?
    }
}

#[cfg(test)]
mod tests {
    use futures::executor::block_on;

    use super::*;
    use crate::os::{AppQuery, System};

    #[test]
    fn open_finder() {
        let initial_app = System::current_app();
        assert!(block_on(App::open("com.apple.finder")).is_ok());
        if let Ok(Some(restore_id)) = initial_app {
            block_on(App::open(&restore_id)).unwrap();
        }
    }

    #[test]
    fn open_fake_app() {
        let result = block_on(App::open("com.test.fake"));
        assert_eq!(
            result.unwrap_err().to_string(),
            "could not find app with bundle id 'com.test.fake'"
        );
    }
}
