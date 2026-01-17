use crate::model::{
    error::UsecaseError,
    problem::{
        CreateNormalProblemData, NormalProblemDto, NormalProblemsDto, ProblemGetQueryData,
        ProblemOrderByData, UpdateNormalProblemData,
    },
};
use domain::{
    model::problem::{CreateNormalProblem, ProblemGetQuery, ProblemOrderBy, UpdateNormalProblem},
    repository::{
        problem::ProblemRepository, procedure::ProcedureRepository, session::SessionRepository,
        testcase::TestcaseRepository, user::UserRepository,
    },
};
use judge_core::{
    logic::{
        problem_presets::normal_judge::create_normal_judge_procedure,
        registered_procedure_remover::remove, writer_schema_registerer::register,
    },
    model::{
        dep_name_repository::DepNameRepository, problem_registry::ProblemRegistryServer,
        procedure::registered::Procedure,
    },
};
use validator::Validate;

#[derive(Clone)]
pub struct ProblemService<
    PR: ProblemRepository,
    UR: UserRepository,
    SR: SessionRepository,
    TR: TestcaseRepository,
    PRC: ProcedureRepository,
    PRS: ProblemRegistryServer,
    DNR: DepNameRepository<i64>,
> {
    problem_repository: PR,
    user_repository: UR,
    session_repository: SR,
    testcase_repository: TR,
    procedure_repository: PRC,
    problem_registry_server: PRS,
    dep_name_repository: DNR,
}

impl<
    PR: ProblemRepository,
    UR: UserRepository,
    SR: SessionRepository,
    TR: TestcaseRepository,
    PRC: ProcedureRepository,
    PRS: ProblemRegistryServer,
    DNR: DepNameRepository<i64>,
> ProblemService<PR, UR, SR, TR, PRC, PRS, DNR>
{
    pub fn new(
        problem_repository: PR,
        user_repository: UR,
        session_repository: SR,
        testcase_repository: TR,
        procedure_repository: PRC,
        problem_registry_server: PRS,
        dep_name_repository: DNR,
    ) -> Self {
        Self {
            problem_repository,
            user_repository,
            session_repository,
            testcase_repository,
            procedure_repository,
            problem_registry_server,
            dep_name_repository,
        }
    }
}

impl<
    PR: ProblemRepository,
    UR: UserRepository,
    SR: SessionRepository,
    TR: TestcaseRepository,
    PRC: ProcedureRepository,
    PRS: ProblemRegistryServer,
    DNR: DepNameRepository<i64>,
> ProblemService<PR, UR, SR, TR, PRC, PRS, DNR>
{
    pub async fn get_problem(
        &self,
        session_id: Option<&str>,
        problem_id: String,
    ) -> anyhow::Result<NormalProblemDto, UsecaseError> {
        let problem_id: i64 = problem_id
            .parse()
            .map_err(|_| UsecaseError::ValidateError)?;

        let problem = self
            .problem_repository
            .get_problem(problem_id)
            .await
            .map_err(UsecaseError::internal_server_error_map())?
            .ok_or(UsecaseError::NotFound)?;

        if !problem.is_public {
            let session_id = session_id.ok_or(UsecaseError::NotFound)?;

            let display_id: i64 = self
                .session_repository
                .get_display_id_by_session_id(&session_id)
                .await
                .map_err(UsecaseError::internal_server_error_map())?
                .ok_or(UsecaseError::NotFound)?;

            if display_id != problem.author_id {
                return Err(UsecaseError::NotFound);
            }
        }

        let testcases = self
            .testcase_repository
            .get_testcases(problem_id)
            .await
            .map_err(UsecaseError::internal_server_error_map())?;

        Ok(NormalProblemDto {
            id: problem.id.to_string(),
            author_id: problem.author_id.to_string(),
            title: problem.title,
            statement: problem.statement,
            time_limit_ms: problem.time_limit_ms,
            memory_limit_kib: problem.memory_limit_kib,
            difficulty: problem.difficulty,
            is_public: problem.is_public,
            solved_count: problem.solved_count,
            testcases: testcases.into_iter().map(|x| x.into()).collect(),
            created_at: problem.created_at,
            updated_at: problem.updated_at,
        })
    }

    pub async fn get_problems_by_query(
        &self,
        session_id: Option<&str>,
        query: ProblemGetQueryData,
    ) -> anyhow::Result<NormalProblemsDto, UsecaseError> {
        let display_id = match session_id {
            Some(session_id) => self
                .session_repository
                .get_display_id_by_session_id(&session_id)
                .await
                .map_err(UsecaseError::internal_server_error_map())?,
            None => None,
        };

        let user_query = query.user_query.map_or(Ok(None), |user_id_str| {
            let user_id: i64 = user_id_str
                .parse()
                .map_err(|_| UsecaseError::ValidateError)?;
            Ok(Some(user_id))
        })?;

        let query = ProblemGetQuery {
            user_id: display_id,
            user_name: query.user_name,
            user_query: user_query,
            limit: query.limit.unwrap_or(50),
            offset: query.offset.unwrap_or(0),
            order_by: match query.order_by {
                ProblemOrderByData::CreatedAtAsc => ProblemOrderBy::CreatedAtAsc,
                ProblemOrderByData::CreatedAtDesc => ProblemOrderBy::CreatedAtDesc,
                ProblemOrderByData::UpdatedAtAsc => ProblemOrderBy::UpdatedAtAsc,
                ProblemOrderByData::UpdatedAtDesc => ProblemOrderBy::UpdatedAtDesc,
                ProblemOrderByData::DifficultyAsc => ProblemOrderBy::DifficultyAsc,
                ProblemOrderByData::DifficultyDesc => ProblemOrderBy::DifficultyDesc,
            },
        };

        let total = self
            .problem_repository
            .get_problems_by_query_count(query.clone())
            .await
            .map_err(UsecaseError::internal_server_error_map())?;

        let problems = self
            .problem_repository
            .get_problems_by_query(query)
            .await
            .map_err(UsecaseError::internal_server_error_map())?;

        Ok(NormalProblemsDto {
            total: total,
            problems: problems.into_iter().map(|p| p.into()).collect(),
        })
    }

    pub async fn update_problem(
        &self,
        session_id: Option<&str>,
        problem_id: String,
        body: UpdateNormalProblemData,
    ) -> anyhow::Result<NormalProblemDto, UsecaseError> {
        body.validate().map_err(|_| UsecaseError::ValidateError)?;

        let problem_id: i64 = problem_id
            .parse()
            .map_err(|_| UsecaseError::ValidateError)?;

        let problem = self
            .problem_repository
            .get_problem(problem_id)
            .await
            .map_err(UsecaseError::internal_server_error_map())?
            .ok_or(UsecaseError::NotFound)?;

        let display_id = match session_id {
            Some(session_id) => self
                .session_repository
                .get_display_id_by_session_id(session_id)
                .await
                .map_err(UsecaseError::internal_server_error_map())?,
            None => None,
        };

        if display_id.is_none_or(|id| id != problem.author_id) {
            if problem.is_public {
                return Err(UsecaseError::Forbidden);
            } else {
                return Err(UsecaseError::NotFound);
            }
        }

        self.problem_repository
            .update_problem(
                problem_id,
                UpdateNormalProblem {
                    title: body.title.unwrap_or(problem.title),
                    is_public: body.is_public.unwrap_or(problem.is_public),
                    difficulty: body.difficulty.unwrap_or(problem.difficulty),
                    statement: body.statement.unwrap_or(problem.statement),
                    time_limit_ms: body.time_limit_ms.unwrap_or(problem.time_limit_ms),
                    memory_limit_kib: body.memory_limit_kib.unwrap_or(problem.memory_limit_kib),
                },
            )
            .await
            .map_err(UsecaseError::internal_server_error_map())?;

        let problem = self
            .problem_repository
            .get_problem(problem_id)
            .await
            .map_err(UsecaseError::internal_server_error_map())?
            .ok_or(UsecaseError::NotFound)?;

        Ok(problem.into())
    }

    pub async fn create_problem(
        &self,
        session_id: Option<&str>,
        body: CreateNormalProblemData,
    ) -> anyhow::Result<NormalProblemDto, UsecaseError> {
        body.validate().map_err(|_| UsecaseError::ValidateError)?;

        let display_id = match session_id {
            Some(session_id) => self
                .session_repository
                .get_display_id_by_session_id(session_id)
                .await
                .map_err(UsecaseError::internal_server_error_map())?
                .ok_or(UsecaseError::Forbidden)?,
            None => return Err(UsecaseError::Forbidden),
        };

        let user = self
            .user_repository
            .get_user_by_display_id(display_id)
            .await
            .map_err(UsecaseError::internal_server_error_map())?
            .ok_or(UsecaseError::Forbidden)?;

        match user.role {
            domain::model::user::UserRole::Admin | domain::model::user::UserRole::TrapUser => {}
            _ => {
                return Err(UsecaseError::Forbidden);
            }
        }

        let problem_id = self
            .problem_repository
            .create_problem(CreateNormalProblem {
                author_id: display_id,
                title: body.title,
                statement: body.statement,
                time_limit_ms: body.time_limit_ms,
                memory_limit_kib: body.memory_limit_kib,
                difficulty: body.difficulty,
            })
            .await
            .map_err(UsecaseError::internal_server_error_map())?;

        let procedure = create_normal_judge_procedure(vec![])
            .map_err(UsecaseError::internal_server_error_map())?;

        let registered_procedure = register(
            procedure,
            self.problem_registry_server.clone(),
            self.dep_name_repository.clone(),
            problem_id,
        )
        .await
        .map_err(UsecaseError::internal_server_error_map())?;

        if self
            .procedure_repository
            .create_procedure(problem_id, registered_procedure)
            .await
            .is_err()
        {
            let _ = self.problem_repository.delete_problem(problem_id).await;
            return Err(UsecaseError::internal_server_error_msg(
                "failed to create initial procedure for problem",
            ));
        }

        let problem = self
            .problem_repository
            .get_problem(problem_id)
            .await
            .map_err(UsecaseError::internal_server_error_map())?
            .ok_or(UsecaseError::NotFound)?;

        Ok(problem.into())
    }

    pub async fn delete_problem(
        &self,
        session_id: Option<&str>,
        problem_id: String,
    ) -> anyhow::Result<(), UsecaseError> {
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

        if user_id.is_none_or(|id| id != problem.author_id) {
            if problem.is_public {
                return Err(UsecaseError::Forbidden);
            } else {
                return Err(UsecaseError::NotFound);
            }
        }

        // Delete testcases
        self.testcase_repository
            .delete_testcases(problem_id)
            .await
            .map_err(UsecaseError::internal_server_error_map())?;

        // Delete procedure resources from problem registry (S3) and procedure from database
        if let Some(procedure) = self
            .procedure_repository
            .get_procedure(problem_id)
            .await
            .map_err(UsecaseError::internal_server_error_map())?
        {
            remove(procedure, self.problem_registry_server.clone())
                .await
                .map_err(UsecaseError::internal_server_error_map())?;
        }

        self.procedure_repository
            .delete_procedure(problem_id)
            .await
            .map_err(UsecaseError::internal_server_error_map())?;

        // Delete dep_names
        self.dep_name_repository
            .remove_many(problem_id)
            .await
            .map_err(UsecaseError::internal_server_error_map())?;

        // Finally, delete the problem itself
        self.problem_repository
            .delete_problem(problem_id)
            .await
            .map_err(UsecaseError::internal_server_error_map())?;

        Ok(())
    }
}
