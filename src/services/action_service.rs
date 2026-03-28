use super::group_service::GroupService;
use crate::models::Action;

#[derive(Clone)]
pub struct ActionService {
    group_service: GroupService,
}

impl ActionService {
    pub fn new(group_service: GroupService) -> Self {
        Self { group_service }
    }

    pub async fn execute(&self, action: &Action) {
        match action {
            Action::OpenGroup { group_id } => self.group_service.open(*group_id).await,
        }
    }
}
