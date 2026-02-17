use domain::{
    model::icon::{Icon, IconId},
    repository::icon::IconRepository,
};

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
    pub async fn get_icon(&self, id: IconId) -> anyhow::Result<Icon, UsecaseError> {
        let icon = self
            .icon_repository
            .get_icon(id)
            .await
            .map_err(UsecaseError::internal_server_error_map())?
            .ok_or(UsecaseError::NotFound)?;

        Ok(icon)
    }
}
