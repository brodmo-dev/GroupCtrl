use crate::models::Action;
use crate::os::App;
use crate::services::ConfigReader;
use crate::services::group_service::GroupService;

#[derive(Clone)]
pub struct ActionService {
    group_service: GroupService,
}

impl ActionService {
    pub fn new(config_reader: ConfigReader) -> Self {
        Self {
            group_service: GroupService::new(config_reader),
        }
    }
    /// Returns `None` if an app was opened, or `Some(apps)` if the caller
    /// should show the launcher popup.
    /// TODO this doesn't make any sense, we don't want a return type here
    pub async fn execute(&self, action: &Action) -> Option<Vec<App>> {
        match action {
            Action::OpenGroup { group_id } => self.group_service.open(*group_id).await,
        }
    }
}
