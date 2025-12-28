use uuid::Uuid;

use crate::models::Config;
use crate::services::ConfigService;

#[derive(Default)]
pub struct GroupService {}

impl GroupService {
    pub fn open(&self, config_service: &ConfigService, group_id: Uuid) {
        let apps = config_service.group_apps(group_id);
    }
}
