use anyhow::{Context, anyhow, bail};
use block2::RcBlock;
use futures::StreamExt;
use log::info;
use objc2_app_kit::{NSWorkspace, NSWorkspaceOpenConfiguration};
use objc2_foundation::{NSError, NSString, NSURL};

use super::app::App;
use crate::os::Openable;

impl Openable for App {
    async fn open(&self) -> anyhow::Result<()> {
        info!("opening app {self}");
        let Some(ref path) = self.app_path else {
            bail!("could not find app with bundle id '{}'", self.bundle_id);
        };
        let app_url = NSURL::fileURLWithPath(&NSString::from_str(path));
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
        let app = App::from("com.apple.finder".to_string());
        assert!(block_on(app.open()).is_ok());
        if let Ok(Some(restore)) = initial_app {
            block_on(restore.open()).unwrap();
        }
    }

    #[test]
    fn open_fake_app() {
        let fake_app = App::from("com.test.fake".to_string());
        let result = block_on(fake_app.open());
        assert_eq!(
            result.unwrap_err().to_string(),
            "could not find app with bundle id 'com.test.fake'"
        );
    }
}
