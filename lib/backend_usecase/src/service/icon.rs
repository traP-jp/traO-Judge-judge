use domain::{model::icon::Icon, repository::icon::IconRepository};
use uuid::Uuid;

#[derive(Clone)]
pub struct IconService<IR: IconRepository> {
    icon_repository: IR,
}
impl<IR: IconRepository> IconService<IR> {
    pub fn new(icon_repository: IR) -> Self {
        Self { icon_repository }
    }
}

#[derive(Debug)]
pub enum IconServiceError {
    NotFound,
    InternalServerError,
}

impl<IR: IconRepository> IconService<IR> {
    pub async fn get_icon(&self, id: String) -> anyhow::Result<Icon, IconServiceError> {
        let id = match Uuid::parse_str(&id) {
            Ok(id) => id,
            Err(_) => return Err(IconServiceError::NotFound),
        };

        let icon = self
            .icon_repository
            .get_icon(id)
            .await
            .map_err(|_| IconServiceError::InternalServerError)?
            .ok_or(IconServiceError::NotFound)?;

        Ok(icon)
    }
}
