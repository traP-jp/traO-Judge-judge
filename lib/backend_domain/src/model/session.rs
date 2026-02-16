use crate::model::user::{UserDisplayId, UserId};

#[derive(Debug, Clone)]
pub struct SessionUser {
    pub user_id: UserId,
    pub display_id: UserDisplayId,
}
