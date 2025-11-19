use crate::model::{
    error::UsecaseError,
    submission::{
        CreateSubmissionData, JudgeResultDto, SubmissionDto, SubmissionGetQueryData,
        SubmissionOrderByData, SubmissionSummaryDto, SubmissionsDto,
    },
};
use domain::{
    model::submission::{
        CreateJudgeResult, CreateSubmission, SubmissionGetQuery, SubmissionOrderBy,
        UpdateSubmission,
    },
    repository::{
        language::LanguageRepository, problem::ProblemRepository, procedure::ProcedureRepository,
        session::SessionRepository, submission::SubmissionRepository, testcase::TestcaseRepository,
    },
};
use judge_core::{
    constant::label::single_judge,
    model::{
        dep_name_repository::DepNameRepository,
        judge::{JudgeRequest, JudgeService},
        judge_output::{ExecutionJobResult, ExecutionResult},
    },
};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Clone)]
pub struct SubmissionService<
    SeR: SessionRepository + Send + Sync + 'static,
    SuR: SubmissionRepository + Send + Sync + 'static,
    PR: ProblemRepository + Send + Sync + 'static,
    PcR: ProcedureRepository + Send + Sync + 'static,
    TR: TestcaseRepository + Send + Sync + 'static,
    LR: LanguageRepository + Send + Sync + 'static,
    DNR: DepNameRepository<i64> + Send + Sync + 'static,
    JS: JudgeService + Send + Sync + 'static,
> {
    session_repository: SeR,
    submission_repository: SuR,
    problem_repository: PR,
    procedure_repository: PcR,
    testcase_repository: TR,
    language_repository: LR,
    dep_name_repository: DNR,
    judge_service: JS,
}

impl<
    SeR: SessionRepository + Send + Sync + 'static,
    SuR: SubmissionRepository + Send + Sync + 'static,
    PR: ProblemRepository + Send + Sync + 'static,
    PcR: ProcedureRepository + Send + Sync + 'static,
    TR: TestcaseRepository + Send + Sync + 'static,
    LR: LanguageRepository + Send + Sync + 'static,
    DNR: DepNameRepository<i64> + Send + Sync + 'static,
    JS: JudgeService + Send + Sync + 'static,
> SubmissionService<SeR, SuR, PR, PcR, TR, LR, DNR, JS>
{
    pub fn new(
        session_repository: SeR,
        submission_repository: SuR,
        problem_repository: PR,
        procedure_repository: PcR,
        testcase_repository: TR,
        language_repository: LR,
        dep_name_repository: DNR,
        judge_service: JS,
    ) -> Self {
        Self {
            session_repository,
            submission_repository,
            problem_repository,
            procedure_repository,
            testcase_repository,
            language_repository,
            dep_name_repository,
            judge_service,
        }
    }
}

impl<
    SeR: SessionRepository + Send + Sync + 'static,
    SuR: SubmissionRepository + Send + Sync + 'static,
    PR: ProblemRepository + Send + Sync + 'static,
    PcR: ProcedureRepository + Send + Sync + 'static,
    TR: TestcaseRepository + Send + Sync + 'static,
    LR: LanguageRepository + Send + Sync + 'static,
    DNR: DepNameRepository<i64> + Send + Sync + 'static,
    JS: JudgeService + Send + Sync + 'static,
> SubmissionService<SeR, SuR, PR, PcR, TR, LR, DNR, JS>
{
    pub async fn get_submission(
        &self,
        session_id: Option<&str>,
        submission_id: String,
    ) -> anyhow::Result<SubmissionDto, UsecaseError> {
        let submission_id =
            Uuid::parse_str(&submission_id).map_err(|_| UsecaseError::ValidateError)?;

        let submission = self
            .submission_repository
            .get_submission(submission_id)
            .await
            .map_err(UsecaseError::internal_server_error_map())?
            .ok_or(UsecaseError::NotFound)?;

        let problem = self
            .problem_repository
            .get_problem(submission.problem_id)
            .await
            .map_err(UsecaseError::internal_server_error_map())?
            .ok_or(UsecaseError::NotFound)?;

        if !problem.is_public {
            let session_id = session_id.ok_or(UsecaseError::NotFound)?;

            let display_id = self
                .session_repository
                .get_display_id_by_session_id(&session_id)
                .await
                .map_err(UsecaseError::internal_server_error_map())?
                .ok_or(UsecaseError::NotFound)?;

            if display_id != problem.author_id {
                return Err(UsecaseError::NotFound);
            }
        }

        let judge_results = self
            .submission_repository
            .get_submission_results(submission_id)
            .await
            .map_err(UsecaseError::internal_server_error_map())?;

        Ok(SubmissionDto {
            id: submission.id.to_string(),
            user_id: submission.user_id.to_string(),
            user_name: submission.user_name,
            problem_id: submission.problem_id.to_string(),
            problem_title: submission.problem_title,
            submitted_at: submission.submitted_at,
            language_id: submission.language_id.to_string(),
            total_score: submission.total_score,
            max_time_ms: submission.max_time_ms,
            max_memory_mib: submission.max_memory_mib,
            code_length: submission.source.len() as i32,
            overall_judge_status: submission.overall_judge_status,
            judge_results: judge_results
                .into_iter()
                .map(|testcase| JudgeResultDto {
                    testcase_id: testcase.testcase_id.to_string(),
                    testcase_name: testcase.testcase_name,
                    judge_status: testcase.judge_status,
                    score: testcase.score,
                    time_ms: testcase.time_ms,
                    memory_mib: testcase.memory_mib,
                })
                .collect(),
        })
    }

    pub async fn get_submissions(
        &self,
        session_id: Option<&str>,
        query: SubmissionGetQueryData,
    ) -> anyhow::Result<SubmissionsDto, UsecaseError> {
        let user_id = match session_id {
            Some(session_id) => self
                .session_repository
                .get_display_id_by_session_id(&session_id)
                .await
                .map_err(UsecaseError::internal_server_error_map())?,
            None => None,
        };

        let language_id = query.language_id.map_or(Ok(None), |lang_id_str| {
            let lang_id: i64 = lang_id_str
                .parse()
                .map_err(|_| UsecaseError::ValidateError)?;
            Ok(Some(lang_id))
        })?;

        let problem_id = query.problem_id.map_or(Ok(None), |prob_id_str| {
            let prob_id: i64 = prob_id_str
                .parse()
                .map_err(|_| UsecaseError::ValidateError)?;
            Ok(Some(prob_id))
        })?;

        let user_query = query.user_query.map_or(Ok(None), |user_id_str| {
            let user_id: i64 = user_id_str
                .parse()
                .map_err(|_| UsecaseError::ValidateError)?;
            Ok(Some(user_id))
        })?;

        let query = SubmissionGetQuery {
            user_id: user_id,
            limit: query.limit.unwrap_or(50),
            offset: query.offset.unwrap_or(0),
            judge_status: query.judge_status,
            language_id: language_id,
            user_name: query.user_name,
            user_query: user_query,
            order_by: match query.order_by {
                SubmissionOrderByData::SubmittedAtAsc => SubmissionOrderBy::SubmittedAtAsc,
                SubmissionOrderByData::SubmittedAtDesc => SubmissionOrderBy::SubmittedAtDesc,
                SubmissionOrderByData::TimeConsumptionAsc => SubmissionOrderBy::TimeConsumptionAsc,
                SubmissionOrderByData::TimeConsumptionDesc => {
                    SubmissionOrderBy::TimeConsumptionDesc
                }
                SubmissionOrderByData::ScoreAsc => SubmissionOrderBy::ScoreAsc,
                SubmissionOrderByData::ScoreDesc => SubmissionOrderBy::ScoreDesc,
                SubmissionOrderByData::MemoryConsumptionAsc => {
                    SubmissionOrderBy::MemoryConsumptionAsc
                }
                SubmissionOrderByData::MemoryConsumptionDesc => {
                    SubmissionOrderBy::MemoryConsumptionDesc
                }
                SubmissionOrderByData::CodeLengthAsc => SubmissionOrderBy::CodeLengthAsc,
                SubmissionOrderByData::CodeLengthDesc => SubmissionOrderBy::CodeLengthDesc,
            },
            problem_id: problem_id,
        };

        let total = self
            .submission_repository
            .get_submissions_count_by_query(query.clone())
            .await
            .map_err(UsecaseError::internal_server_error_map())?;

        let submissions = self
            .submission_repository
            .get_submissions_by_query(query)
            .await
            .map_err(UsecaseError::internal_server_error_map())?;

        Ok(SubmissionsDto {
            total: total,
            submissions: submissions.into_iter().map(|s| s.into()).collect(),
        })
    }

    pub async fn create_submission(
        self: &std::sync::Arc<Self>,
        session_id: Option<&str>,
        problem_id: String,
        body: CreateSubmissionData,
    ) -> anyhow::Result<SubmissionDto, UsecaseError> {
        let problem_id: i64 = problem_id
            .parse()
            .map_err(|_| UsecaseError::ValidateError)?;

        let display_id = match session_id {
            Some(session_id) => self
                .session_repository
                .get_display_id_by_session_id(session_id)
                .await
                .map_err(UsecaseError::internal_server_error_map())?
                .ok_or(UsecaseError::Forbidden)?,
            None => return Err(UsecaseError::Forbidden),
        };

        let problem = self
            .problem_repository
            .get_problem(problem_id)
            .await
            .map_err(UsecaseError::internal_server_error_map())?
            .ok_or(UsecaseError::NotFound)?;

        if !problem.is_public && problem.author_id != display_id {
            return Err(UsecaseError::NotFound);
        }

        let language_id: i32 = body
            .language_id
            .parse()
            .map_err(|_| UsecaseError::ValidateError)?;

        let language = self
            .language_repository
            .id_to_language(language_id)
            .await
            .map_err(UsecaseError::internal_server_error_map())?
            .ok_or(UsecaseError::ValidateError)?;

        let procedure = self
            .procedure_repository
            .get_procedure(problem_id)
            .await
            .map_err(UsecaseError::internal_server_error_map())?
            .ok_or_else(|| {
                UsecaseError::internal_server_error_msg(
                    "procedure not found for problem when creating submission",
                )
            })?;

        let submission = CreateSubmission {
            problem_id,
            user_id: display_id,
            language_id: language_id,
            source: body.source.clone(),
            judge_status: "WJ".to_string(),
            total_score: 0,
            max_time_ms: 0,
            max_memory_mib: 0,
        };

        let submission_id = self
            .submission_repository
            .create_submission(submission)
            .await
            .map_err(UsecaseError::internal_server_error_map())?;

        let mut runtime_texts = HashMap::new();
        runtime_texts.insert(
            single_judge::SUBMISSION_SOURCE.to_string(),
            body.source.clone(),
        );
        runtime_texts.insert(single_judge::LANGUAGE_TAG.to_string(), language.clone());
        runtime_texts.insert(
            single_judge::TIME_LIMIT_MS.to_string(),
            problem.time_limit_ms.to_string(),
        );
        runtime_texts.insert(
            single_judge::MEMORY_LIMIT_KIB.to_string(),
            (problem.memory_limit_mib as i64 * 1024).to_string(),
        );

        let self_clone = std::sync::Arc::clone(self);

        tracing::info!(
            %submission_id,
            problem_id,
            user_id = display_id,
            language = %language,
            "spawning judge task"
        );

        tokio::spawn(async move {
            tracing::info!(%submission_id, problem_id, "judge task started");
            if let Err(e) = self_clone
                .async_judge_submission(submission_id, problem_id, procedure, runtime_texts)
                .await
            {
                match e {
                    UsecaseError::InternalServerError {
                        message,
                        file,
                        line,
                        column,
                    } => {
                        tracing::error!(
                            %submission_id,
                            problem_id,
                            %message,
                            file,
                            line,
                            column,
                            "judge task failed"
                        );
                    }
                    other => {
                        tracing::warn!(
                            %submission_id,
                            problem_id,
                            error = ?other,
                            "judge task failed"
                        );
                    }
                }
            }
        });

        self.get_submission(session_id, submission_id.to_string())
            .await
    }

    #[tracing::instrument(skip(self, procedure, runtime_texts), fields(%submission_id, problem_id))]
    async fn async_judge_submission(
        &self,
        submission_id: Uuid,
        problem_id: i64,
        procedure: judge_core::model::procedure::registered::Procedure,
        runtime_texts: HashMap<String, String>,
    ) -> anyhow::Result<(), UsecaseError> {
        let judge_response = self
            .judge_service
            .judge(JudgeRequest {
                procedure,
                runtime_texts,
            })
            .await
            .map_err(UsecaseError::internal_server_error_map())?;

        let keys = judge_response.keys().cloned().collect::<Vec<_>>();
        let testcase_names = self
            .dep_name_repository
            .get_many(keys)
            .await
            .map_err(UsecaseError::internal_server_error_map())?;

        let testcases = self
            .testcase_repository
            .get_testcases(problem_id)
            .await
            .map_err(UsecaseError::internal_server_error_map())?;

        let name_to_id = testcases
            .into_iter()
            .map(|tc| (tc.name, tc.id))
            .collect::<HashMap<_, _>>();

        let mut total_score: i64 = 0; // summary phase から取ってくる
        let mut max_time_ms: i32 = 0; // summary phase から取ってくる
        let mut max_memory_mib: i32 = 0; // summary phase から取ってくる
        let mut overall_status = "IE".to_string(); // summary phase から -> compile phase から取る
        let mut testcase_results: Vec<CreateJudgeResult> = Vec::new();

        for (dep_id, result) in judge_response.into_iter() {
            match result {
                ExecutionJobResult::ExecutionResult(exec) => match exec {
                    ExecutionResult::Displayable(res) => {
                        let testcase_name = testcase_names
                            .get(&dep_id)
                            .cloned()
                            .flatten()
                            .unwrap_or_default();

                        if testcase_name
                            .starts_with(judge_core::constant::job_name::TEST_PHASE_PREFIX)
                        {
                            let testcase_name = testcase_name
                                .strip_prefix(judge_core::constant::job_name::TEST_PHASE_PREFIX)
                                .unwrap_or(testcase_name.as_str())
                                .to_string();

                            let testcase_id =
                                name_to_id.get(&testcase_name).cloned().unwrap_or_default();

                            testcase_results.push(CreateJudgeResult {
                                submission_id,
                                testcase_id,
                                testcase_name,
                                judge_status: format!("{:?}", res.status),
                                score: res.score,
                                time_ms: res.time as i32,
                                memory_mib: (res.memory / 1024.) as i32,
                            });
                        }
                        if testcase_name == judge_core::constant::job_name::SUMMARY_PHASE {
                            total_score = res.score;
                            max_time_ms = res.time as i32;
                            max_memory_mib = (res.memory / 1024.) as i32;
                            overall_status = format!("{:?}", res.status);
                        }
                        if testcase_name == judge_core::constant::job_name::COMPILE_PHASE
                            && overall_status == "IE"
                        {
                            overall_status = format!("{:?}", res.status);
                        }
                    }
                    ExecutionResult::Hidden(_res) => {
                        // todo
                    }
                },
                ExecutionJobResult::EarlyExit => {}
            }
        }

        let testcase_count = testcase_results.len();
        let overall_status_str = overall_status.clone();

        self.submission_repository
            .update_submission(
                submission_id,
                UpdateSubmission {
                    total_score,
                    max_time_ms,
                    max_memory_mib,
                    judge_status: overall_status,
                },
            )
            .await
            .map_err(UsecaseError::internal_server_error_map())?;

        self.submission_repository
            .create_judge_results(testcase_results)
            .await
            .map_err(UsecaseError::internal_server_error_map())?;

        tracing::info!(
            %submission_id,
            problem_id,
            testcase_count,
            total_score,
            max_time_ms,
            max_memory_mib,
            overall_status = %overall_status_str,
            "judge finished"
        );

        Ok(())
    }
}
