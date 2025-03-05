use crate::model::editorials::EditorialDto;
use domain::repository::{editorials::EditorialsRepository, session::SessionRepository};

#[derive(Clone)]
pub struct EditorialsService<SeR: SessionRepository, ER: EditorialsRepository> {
    session_repository: SeR,
    editorials_repository: ER,
}

impl<SeR: SessionRepository, ER: EditorialsRepository> EditorialsService<SeR, ER> {
    pub fn new(session_repository: SeR, editorials_repository: ER) -> Self {
        Self {
            session_repository,
            editorials_repository,
        }
    }
}

#[derive(Debug)]
pub enum EditorialsError {
    ValidateError,
    Forbidden,
    NotFound,
    InternalServerError,
}

impl<SeR: SessionRepository, ER: EditorialsRepository> EditorialsService<SeR, ER> {
    pub async fn get_editorial(
        &self,
        session_id: Option<String>,
        editorial_id: i64,
    ) -> anyhow::Result<EditorialDto, EditorialsError> {
        let editorial = self
            .editorials_repository
            .get_editorial(editorial_id)
            .await
            .map_err(|_| EditorialsError::InternalServerError)?
            .ok_or(EditorialsError::NotFound)?;

        if !editorial.is_public {
            let session_id = session_id.ok_or(EditorialsError::Forbidden)?;
            let display_id = self
                .session_repository
                .get_display_id_by_session_id(&session_id)
                .await
                .map_err(|_| EditorialsError::InternalServerError)?
                .ok_or(EditorialsError::Forbidden)?;

            if display_id != editorial.author_id {
                return Err(EditorialsError::Forbidden);
            }
        }
        Ok(editorial.into())
    }
}
