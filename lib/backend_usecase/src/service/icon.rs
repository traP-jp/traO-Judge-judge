use domain::{model::icon::Icon, repository::icon::IconRepository};
use uuid::Uuid;

use crate::model::error::UsecaseError;

#[derive(Clone)]
pub struct IconService<IR: IconRepository> {
    icon_repository: IR,
}
impl<IR: IconRepository> IconService<IR> {
    pub fn new(icon_repository: IR) -> Self {
        Self { icon_repository }
    }
}

impl<IR: IconRepository> IconService<IR> {
    pub async fn get_icon(&self, id: String) -> anyhow::Result<Icon, UsecaseError> {
        let id = match Uuid::parse_str(&id) {
            Ok(id) => id,
            Err(_) => return Err(UsecaseError::NotFound),
        };

        let icon = self
            .icon_repository
            .get_icon(id)
            .await
            .map_err(UsecaseError::internal_server_error)?
            .ok_or(UsecaseError::NotFound)?;

        Ok(icon)
    }
}
