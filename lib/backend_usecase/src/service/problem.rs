use crate::model::problem::{CreateNormalProblemData, NormalProblemDto, UpdateNormalProblemData};
use domain::{
    model::problem::{CreateNormalProblem, UpdateNormalProblem},
    repository::{problem::ProblemRepository, session::SessionRepository},
};

#[derive(Clone)]
pub struct ProblemService<PR: ProblemRepository, SR: SessionRepository> {
    problem_repository: PR,
    session_repository: SR,
}

impl<PR: ProblemRepository, SR: SessionRepository> ProblemService<PR, SR> {
    pub fn new(problem_repository: PR, session_repository: SR) -> Self {
        Self {
            problem_repository,
            session_repository,
        }
    }
}

#[derive(Debug)]
pub enum ProblemError {
    ValidateError,
    Unauthorized,
    Forbidden,
    NotFound,
    InternalServerError,
}

impl<PR: ProblemRepository, SR: SessionRepository> ProblemService<PR, SR> {
    pub async fn get_problem(
        &self,
        session_id: Option<String>,
        problem_id: i64,
    ) -> anyhow::Result<NormalProblemDto, ProblemError> {
        let problem = self
            .problem_repository
            .get_problem(problem_id)
            .await
            .map_err(|_| ProblemError::InternalServerError)?
            .ok_or(ProblemError::NotFound)?;

        if !problem.is_public {
            let session_id = session_id.ok_or(ProblemError::NotFound)?;

            let display_id: i64 = self
                .session_repository
                .get_display_id_by_session_id(&session_id)
                .await
                .map_err(|_| ProblemError::InternalServerError)?
                .ok_or(ProblemError::NotFound)?;

            if display_id != problem.author_id {
                return Err(ProblemError::Forbidden);
            }
        }

        Ok(problem.into())
    }

    pub async fn update_problem(
        &self,
        session_id: &str,
        problem_id: i64,
        body: UpdateNormalProblemData,
    ) -> anyhow::Result<NormalProblemDto, ProblemError> {
        let problem = self
            .problem_repository
            .get_problem(problem_id)
            .await
            .map_err(|_| ProblemError::InternalServerError)?
            .ok_or(ProblemError::NotFound)?;

        let display_id = self
            .session_repository
            .get_display_id_by_session_id(&session_id)
            .await
            .map_err(|_| ProblemError::InternalServerError)?
            .ok_or(ProblemError::Unauthorized)?;

        if display_id != problem.author_id {
            return Err(ProblemError::Forbidden);
        }

        self.problem_repository
            .update_problem(
                problem_id,
                UpdateNormalProblem {
                    title: body.title.unwrap_or(problem.title),
                    is_public: body.is_public.unwrap_or(problem.is_public),
                    difficulty: body.difficulty.unwrap_or(problem.difficulty),
                    statement: body.statement,
                    time_limit: body.time_limit.unwrap_or(problem.time_limit),
                    memory_limit: body.memory_limit.unwrap_or(problem.memory_limit),
                },
            )
            .await
            .map_err(|_| ProblemError::InternalServerError)?;

        let problem = self
            .problem_repository
            .get_problem(problem_id)
            .await
            .map_err(|_| ProblemError::InternalServerError)?
            .ok_or(ProblemError::NotFound)?;

        Ok(problem.into())
    }

    pub async fn create_problem(
        &self,
        session_id: &str,
        body: CreateNormalProblemData,
    ) -> anyhow::Result<NormalProblemDto, ProblemError> {
        let display_id = self
            .session_repository
            .get_display_id_by_session_id(&session_id)
            .await
            .map_err(|_| ProblemError::InternalServerError)?
            .ok_or(ProblemError::Unauthorized)?;

        let problem_id = self
            .problem_repository
            .create_problem(CreateNormalProblem {
                author_id: display_id,
                title: body.title,
                statement: body.statement,
                time_limit: body.time_limit,
                memory_limit: body.memory_limit,
                difficulty: body.difficulty,
                judgecode_path: "todo".to_string(),
            })
            .await
            .map_err(|_| ProblemError::InternalServerError)?;

        let problem = self
            .problem_repository
            .get_problem(problem_id)
            .await
            .map_err(|_| ProblemError::InternalServerError)?
            .ok_or(ProblemError::NotFound)?;

        Ok(problem.into())
    }
}
