use crate::models::Action;
use crate::services::ConfigReader;
use crate::services::group_service::GroupService;

pub struct ActionService {
    group_service: GroupService,
}

impl ActionService {
    pub fn new(config_reader: ConfigReader) -> Self {
        Self {
            group_service: GroupService::new(config_reader),
        }
    }
    pub fn execute(&mut self, action: &Action) {
        match action {
            Action::OpenGroup { group_id } => self.group_service.open(*group_id),
        }
    }
}
