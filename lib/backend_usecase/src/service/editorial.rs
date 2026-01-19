use crate::model::{
    editorial::{CreateEditorialData, EditorialDto, EditorialSummaryDto, UpdateEditorialData},
    error::UsecaseError,
};
use domain::{
    model::editorial::{CreateEditorial, EditorialGetQuery, UpdateEditorial},
    repository::{
        editorial::EditorialRepository, problem::ProblemRepository, session::SessionRepository,
    },
};
use uuid::Uuid;

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

impl<SR: SessionRepository, ER: EditorialRepository, PR: ProblemRepository>
    EditorialService<SR, ER, PR>
{
    pub async fn get_editorial(
        &self,
        session_id: Option<&str>,
        editorial_id: String,
    ) -> anyhow::Result<EditorialDto, UsecaseError> {
        let editorial_id =
            Uuid::parse_str(&editorial_id).map_err(|_| UsecaseError::ValidateError)?;

        let user_id = match session_id {
            Some(session_id) => self
                .session_repository
                .get_display_id_by_session_id(session_id)
                .await
                .map_err(UsecaseError::internal_server_error_map())?,
            None => None,
        };

        let editorial = self
            .editorial_repository
            .get_editorial(editorial_id)
            .await
            .map_err(UsecaseError::internal_server_error_map())?
            .ok_or(UsecaseError::NotFound)?;

        if !editorial.is_public && user_id.is_none_or(|x| x != editorial.author_id) {
            return Err(UsecaseError::NotFound);
        }

        let problem = self
            .problem_repository
            .get_problem(editorial.problem_id)
            .await
            .map_err(UsecaseError::internal_server_error_map())?
            .ok_or(UsecaseError::NotFound)?;

        if !problem.is_public && user_id.is_none_or(|x| x != problem.author_id) {
            return Err(UsecaseError::NotFound);
        }

        Ok(editorial.into())
    }

    pub async fn get_editorials(
        &self,
        session_id: Option<&str>,
        problem_id: String,
    ) -> anyhow::Result<Vec<EditorialSummaryDto>, UsecaseError> {
        let problem_id: i64 = problem_id
            .parse()
            .map_err(|_| UsecaseError::ValidateError)?;

        let user_id = match session_id {
            Some(session_id) => self
                .session_repository
                .get_display_id_by_session_id(session_id)
                .await
                .map_err(UsecaseError::internal_server_error_map())?,
            None => None,
        };

        let problem = self
            .problem_repository
            .get_problem(problem_id)
            .await
            .map_err(UsecaseError::internal_server_error_map())?
            .ok_or(UsecaseError::NotFound)?;

        if !problem.is_public && user_id.is_none_or(|x| x != problem.author_id) {
            return Err(UsecaseError::NotFound);
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
            .map_err(UsecaseError::internal_server_error_map())?;

        Ok(editorials.into_iter().map(|x| x.into()).collect())
    }

    pub async fn post_editorial(
        &self,
        session_id: Option<&str>,
        problem_id: String,
        query: CreateEditorialData,
    ) -> anyhow::Result<EditorialDto, UsecaseError> {
        let problem_id: i64 = problem_id
            .parse()
            .map_err(|_| UsecaseError::ValidateError)?;

        let problem = self
            .problem_repository
            .get_problem(problem_id)
            .await
            .map_err(UsecaseError::internal_server_error_map())?
            .ok_or(UsecaseError::NotFound)?;

        let user_id = match session_id {
            Some(session_id) => self
                .session_repository
                .get_display_id_by_session_id(session_id)
                .await
                .map_err(UsecaseError::internal_server_error_map())?,
            None => None,
        };

        if !problem.is_public && user_id.is_none_or(|x| x != problem.author_id) {
            return Err(UsecaseError::NotFound);
        }

        let user_id = user_id.ok_or(UsecaseError::Forbidden)?;

        let editorial = CreateEditorial {
            problem_id,
            author_id: user_id,
            title: query.title,
            statement: query.statement,
            is_public: query.is_public,
        };

        let editorial_id = self
            .editorial_repository
            .create_editorial(editorial)
            .await
            .map_err(UsecaseError::internal_server_error_map())?;

        let editorial = self
            .editorial_repository
            .get_editorial(editorial_id)
            .await
            .map_err(UsecaseError::internal_server_error_map())?
            .ok_or_else(|| {
                UsecaseError::internal_server_error_msg(
                    "failed to retrieve editorial after creation",
                )
            })?;

        Ok(editorial.into())
    }

    pub async fn put_editorial(
        &self,
        session_id: Option<&str>,
        editorial_id: String,
        query: UpdateEditorialData,
    ) -> anyhow::Result<(), UsecaseError> {
        let editorial_id =
            Uuid::parse_str(&editorial_id).map_err(|_| UsecaseError::ValidateError)?;

        let user_id = match session_id {
            Some(session_id) => self
                .session_repository
                .get_display_id_by_session_id(session_id)
                .await
                .map_err(UsecaseError::internal_server_error_map())?,
            None => None,
        };

        let editorial = self
            .editorial_repository
            .get_editorial(editorial_id)
            .await
            .map_err(UsecaseError::internal_server_error_map())?
            .ok_or(UsecaseError::NotFound)?;

        if !editorial.is_public && user_id.is_none_or(|x| x != editorial.author_id) {
            return Err(UsecaseError::NotFound);
        }

        let problem = self
            .problem_repository
            .get_problem(editorial.problem_id)
            .await
            .map_err(UsecaseError::internal_server_error_map())?
            .ok_or(UsecaseError::NotFound)?;

        if !problem.is_public && user_id.is_none_or(|x| x != problem.author_id) {
            return Err(UsecaseError::NotFound);
        }

        let user_id = user_id.ok_or(UsecaseError::Forbidden)?;

        if user_id != editorial.author_id {
            return Err(UsecaseError::Forbidden);
        }

        let editorial = UpdateEditorial {
            id: editorial_id,
            title: query.title,
            statement: query.statement,
            is_public: query.is_public,
        };

        self.editorial_repository
            .update_editorial(editorial)
            .await
            .map_err(UsecaseError::internal_server_error_map())?;

        Ok(())
    }

    pub async fn delete_editorial(
        &self,
        session_id: Option<&str>,
        editorial_id: String,
    ) -> anyhow::Result<(), UsecaseError> {
        let editorial_id =
            Uuid::parse_str(&editorial_id).map_err(|_| UsecaseError::ValidateError)?;

        let user_id = match session_id {
            Some(session_id) => self
                .session_repository
                .get_display_id_by_session_id(session_id)
                .await
                .map_err(UsecaseError::internal_server_error_map())?,
            None => None,
        };

        let editorial = self
            .editorial_repository
            .get_editorial(editorial_id)
            .await
            .map_err(UsecaseError::internal_server_error_map())?
            .ok_or(UsecaseError::NotFound)?;

        if !editorial.is_public && user_id.is_none_or(|x| x != editorial.author_id) {
            return Err(UsecaseError::NotFound);
        }

        let problem = self
            .problem_repository
            .get_problem(editorial.problem_id)
            .await
            .map_err(UsecaseError::internal_server_error_map())?
            .ok_or(UsecaseError::NotFound)?;
        if !problem.is_public && user_id.is_none_or(|x| x != problem.author_id) {
            return Err(UsecaseError::NotFound);
        }

        let user_id = user_id.ok_or(UsecaseError::Forbidden)?;

        if user_id != editorial.author_id {
            return Err(UsecaseError::Forbidden);
        }

        self.editorial_repository
            .delete_editorial(editorial_id)
            .await
            .map_err(UsecaseError::internal_server_error_map())?;

        Ok(())
    }
}
