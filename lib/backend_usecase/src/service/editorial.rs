use crate::model::{
    editorial::{CreateEditorialData, EditorialDto, EditorialSummaryDto, UpdateEditorialData},
    error::UsecaseError,
};
use domain::model::editorial::EditorialId;
use domain::model::problem::ProblemId;
use domain::model::session::SessionUser;
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

impl<SR: SessionRepository, ER: EditorialRepository, PR: ProblemRepository>
    EditorialService<SR, ER, PR>
{
    pub async fn get_editorial(
        &self,
        session_user: Option<SessionUser>,
        editorial_id: EditorialId,
    ) -> anyhow::Result<EditorialDto, UsecaseError> {
        let editorial = self
            .editorial_repository
            .get_editorial(editorial_id)
            .await
            .map_err(UsecaseError::internal_server_error_map())?
            .ok_or(UsecaseError::NotFound)?;

        if !editorial.is_public
            && session_user
                .as_ref()
                .is_none_or(|x| x.display_id != editorial.author_id)
        {
            return Err(UsecaseError::NotFound);
        }

        let problem = self
            .problem_repository
            .get_problem(editorial.problem_id)
            .await
            .map_err(UsecaseError::internal_server_error_map())?
            .ok_or(UsecaseError::NotFound)?;

        if !problem.is_public && session_user.is_none_or(|x| x.display_id != problem.author_id) {
            return Err(UsecaseError::NotFound);
        }

        Ok(editorial.into())
    }

    pub async fn get_editorials(
        &self,
        session_user: Option<SessionUser>,
        problem_id: ProblemId,
    ) -> anyhow::Result<Vec<EditorialSummaryDto>, UsecaseError> {
        let problem = self
            .problem_repository
            .get_problem(problem_id)
            .await
            .map_err(UsecaseError::internal_server_error_map())?
            .ok_or(UsecaseError::NotFound)?;

        if !problem.is_public
            && session_user
                .as_ref()
                .is_none_or(|x| x.display_id != problem.author_id)
        {
            return Err(UsecaseError::NotFound);
        }

        let query = EditorialGetQuery {
            user_id: session_user.as_ref().map(|u| u.display_id),
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
        session_user: Option<SessionUser>,
        problem_id: ProblemId,
        query: CreateEditorialData,
    ) -> anyhow::Result<EditorialDto, UsecaseError> {
        let problem = self
            .problem_repository
            .get_problem(problem_id)
            .await
            .map_err(UsecaseError::internal_server_error_map())?
            .ok_or(UsecaseError::NotFound)?;

        let session_user = session_user.ok_or_else(|| {
            if !problem.is_public {
                UsecaseError::NotFound
            } else {
                UsecaseError::Forbidden
            }
        })?;

        if !problem.is_public && session_user.display_id != problem.author_id {
            return Err(UsecaseError::NotFound);
        }

        let editorial = CreateEditorial {
            problem_id,
            author_id: session_user.display_id,
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
        session_user: Option<SessionUser>,
        editorial_id: EditorialId,
        query: UpdateEditorialData,
    ) -> anyhow::Result<(), UsecaseError> {
        let editorial = self
            .editorial_repository
            .get_editorial(editorial_id)
            .await
            .map_err(UsecaseError::internal_server_error_map())?
            .ok_or(UsecaseError::NotFound)?;
        let session_user = session_user.ok_or_else(|| {
            if !editorial.is_public {
                UsecaseError::NotFound
            } else {
                UsecaseError::Forbidden
            }
        })?;
        if !editorial.is_public && session_user.display_id != editorial.author_id {
            return Err(UsecaseError::NotFound);
        }
        if session_user.display_id != editorial.author_id {
            return Err(UsecaseError::Forbidden);
        }

        let problem = self
            .problem_repository
            .get_problem(editorial.problem_id)
            .await
            .map_err(UsecaseError::internal_server_error_map())?
            .ok_or(UsecaseError::NotFound)?;
        if !problem.is_public && session_user.display_id != problem.author_id {
            return Err(UsecaseError::NotFound);
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
        session_user: Option<SessionUser>,
        editorial_id: EditorialId,
    ) -> anyhow::Result<(), UsecaseError> {
        let editorial = self
            .editorial_repository
            .get_editorial(editorial_id)
            .await
            .map_err(UsecaseError::internal_server_error_map())?
            .ok_or(UsecaseError::NotFound)?;
        let session_user = session_user.ok_or_else(|| {
            if !editorial.is_public {
                UsecaseError::NotFound
            } else {
                UsecaseError::Forbidden
            }
        })?;

        if !editorial.is_public && session_user.display_id != editorial.author_id {
            return Err(UsecaseError::NotFound);
        }
        if session_user.display_id != editorial.author_id {
            return Err(UsecaseError::Forbidden);
        }

        let problem = self
            .problem_repository
            .get_problem(editorial.problem_id)
            .await
            .map_err(UsecaseError::internal_server_error_map())?
            .ok_or(UsecaseError::NotFound)?;
        if !problem.is_public && session_user.display_id != problem.author_id {
            return Err(UsecaseError::NotFound);
        }

        self.editorial_repository
            .delete_editorial(editorial_id)
            .await
            .map_err(UsecaseError::internal_server_error_map())?;

        Ok(())
    }
}
