use crate::model::editorial::{
    CreateEditorialData, EditorialDto, EditorialSummaryDto, UpdateEditorialData,
};
use domain::{
    model::editorial::{CreateEditorial, EditorialGetQuery, UpdateEditorial},
    repository::{
        editorial::EditorialRepository, problem::ProblemRepository, session::SessionRepository,
    },
};

#[derive(Clone)]
pub struct EditorialService<SR: SessionRepository, ER: EditorialRepository, PR: ProblemRepository> {
    session_repository: SR,
    editorial_repository: ER,
    problem_repository: PR,
}

impl<SR: SessionRepository, ER: EditorialRepository, PR: ProblemRepository>
    EditorialService<SR, ER, PR>
{
    pub fn new(session_repository: SR, editorial_repository: ER, problem_repository: PR) -> Self {
        Self {
            session_repository,
            editorial_repository,
            problem_repository,
        }
    }
}

#[derive(Debug)]
pub enum EditorialError {
    ValidateError,
    Forbidden,
    NotFound,
    InternalServerError,
}

impl<SR: SessionRepository, ER: EditorialRepository, PR: ProblemRepository>
    EditorialService<SR, ER, PR>
{
    pub async fn get_editorial(
        &self,
        session_id: Option<&str>,
        editorial_id: i64,
    ) -> anyhow::Result<EditorialDto, EditorialError> {
        let user_id = match session_id {
            Some(session_id) => self
                .session_repository
                .get_display_id_by_session_id(session_id)
                .await
                .map_err(|_| EditorialError::InternalServerError)?,
            None => None,
        };

        let editorial = self
            .editorial_repository
            .get_editorial(editorial_id)
            .await
            .map_err(|_| EditorialError::InternalServerError)?
            .ok_or(EditorialError::NotFound)?;

        if !editorial.is_public && user_id.is_none_or(|x| x != editorial.author_id) {
            return Err(EditorialError::NotFound);
        }

        let problem = self
            .problem_repository
            .get_problem(editorial.problem_id)
            .await
            .map_err(|_| EditorialError::InternalServerError)?
            .ok_or(EditorialError::NotFound)?;

        if !problem.is_public && user_id.is_none_or(|x| x != problem.author_id) {
            return Err(EditorialError::NotFound);
        }

        Ok(editorial.into())
    }

    pub async fn get_editorials(
        &self,
        session_id: Option<&str>,
        problem_id: i64,
    ) -> anyhow::Result<Vec<EditorialSummaryDto>, EditorialError> {
        let user_id = match session_id {
            Some(session_id) => self
                .session_repository
                .get_display_id_by_session_id(session_id)
                .await
                .map_err(|_| EditorialError::InternalServerError)?,
            None => None,
        };

        let problem = self
            .problem_repository
            .get_problem(problem_id)
            .await
            .map_err(|_| EditorialError::InternalServerError)?
            .ok_or(EditorialError::NotFound)?;

        if !problem.is_public && user_id.is_none_or(|x| x != problem.author_id) {
            return Err(EditorialError::NotFound);
        }

        let query = EditorialGetQuery {
            user_id,
            problem_id,
            limit: 50,
            offset: 0,
        };

        let editorials = self
            .editorial_repository
            .get_editorials_by_problem_id(query)
            .await
            .map_err(|_| EditorialError::InternalServerError)?;

        Ok(editorials.into_iter().map(|x| x.into()).collect())
    }

    pub async fn post_editorial(
        &self,
        session_id: Option<&str>,
        problem_id: i64,
        query: CreateEditorialData,
    ) -> anyhow::Result<EditorialDto, EditorialError> {
        let problem = self
            .problem_repository
            .get_problem(problem_id)
            .await
            .map_err(|_| EditorialError::InternalServerError)?
            .ok_or(EditorialError::NotFound)?;

        let user_id = match session_id {
            Some(session_id) => self
                .session_repository
                .get_display_id_by_session_id(session_id)
                .await
                .map_err(|_| EditorialError::InternalServerError)?,
            None => None,
        };

        if !problem.is_public && user_id.is_none_or(|x| x != problem.author_id) {
            return Err(EditorialError::NotFound);
        }

        let user_id = user_id.ok_or(EditorialError::Forbidden)?;

        let editorial = CreateEditorial {
            problem_id,
            author_id: user_id,
            statement: query.statement,
            is_public: query.is_public,
        };

        let editorial_id = self
            .editorial_repository
            .create_editorial(editorial)
            .await
            .map_err(|_| EditorialError::InternalServerError)?;

        let editorial = self
            .editorial_repository
            .get_editorial(editorial_id)
            .await
            .map_err(|_| EditorialError::InternalServerError)?
            .ok_or(EditorialError::InternalServerError)?;

        Ok(editorial.into())
    }

    pub async fn put_editorial(
        &self,
        session_id: Option<&str>,
        editorial_id: i64,
        query: UpdateEditorialData,
    ) -> anyhow::Result<(), EditorialError> {
        let user_id = match session_id {
            Some(session_id) => self
                .session_repository
                .get_display_id_by_session_id(session_id)
                .await
                .map_err(|_| EditorialError::InternalServerError)?,
            None => None,
        };

        let editorial = self
            .editorial_repository
            .get_editorial(editorial_id)
            .await
            .map_err(|_| EditorialError::InternalServerError)?
            .ok_or(EditorialError::NotFound)?;

        if !editorial.is_public && user_id.is_none_or(|x| x != editorial.author_id) {
            return Err(EditorialError::NotFound);
        }

        let problem = self
            .problem_repository
            .get_problem(editorial.problem_id)
            .await
            .map_err(|_| EditorialError::InternalServerError)?
            .ok_or(EditorialError::NotFound)?;

        if !problem.is_public && user_id.is_none_or(|x| x != problem.author_id) {
            return Err(EditorialError::NotFound);
        }

        let user_id = user_id.ok_or(EditorialError::Forbidden)?;

        if user_id != editorial.author_id {
            return Err(EditorialError::Forbidden);
        }

        let editorial = UpdateEditorial {
            id: editorial_id,
            statement: query.statement,
            is_public: query.is_public,
        };

        self.editorial_repository
            .update_editorial(editorial)
            .await
            .map_err(|_| EditorialError::InternalServerError)?;

        Ok(())
    }

    pub async fn delete_editorial(
        &self,
        session_id: Option<&str>,
        editorial_id: i64,
    ) -> anyhow::Result<(), EditorialError> {
        let user_id = match session_id {
            Some(session_id) => self
                .session_repository
                .get_display_id_by_session_id(session_id)
                .await
                .map_err(|_| EditorialError::InternalServerError)?,
            None => None,
        };

        let editorial = self
            .editorial_repository
            .get_editorial(editorial_id)
            .await
            .map_err(|_| EditorialError::InternalServerError)?
            .ok_or(EditorialError::NotFound)?;

        if !editorial.is_public && user_id.is_none_or(|x| x != editorial.author_id) {
            return Err(EditorialError::NotFound);
        }

        let problem = self
            .problem_repository
            .get_problem(editorial.problem_id)
            .await
            .map_err(|_| EditorialError::InternalServerError)?
            .ok_or(EditorialError::NotFound)?;
        if !problem.is_public && user_id.is_none_or(|x| x != problem.author_id) {
            return Err(EditorialError::NotFound);
        }

        let user_id = user_id.ok_or(EditorialError::Forbidden)?;

        if user_id != editorial.author_id {
            return Err(EditorialError::Forbidden);
        }

        self.editorial_repository
            .delete_editorial(editorial_id)
            .await
            .map_err(|_| EditorialError::InternalServerError)?;

        Ok(())
    }
}
