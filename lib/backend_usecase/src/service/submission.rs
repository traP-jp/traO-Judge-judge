use crate::model::submission::{
    CreateSubmissionData, JudgeResultDto, SubmissionDto, SubmissionGetQueryData,
    SubmissionOrderByData, SubmissionSummaryDto, SubmissionsDto,
};
use domain::{
    model::submission::{
        CreateJudgeResult, CreateSubmission, SubmissionGetQuery, SubmissionOrderBy,
        UpdateSubmission,
    },
    repository::{
        language::LanguageRepository, problem::ProblemRepository, procedure::ProcedureRepository,
        session::SessionRepository, submission::SubmissionRepository, testcase::TestcaseRepository,
        user::UserRepository,
    },
};
use judge_core::{
    constant::label::single_judge,
    model::{
        dep_name_repository::DepNameRepository,
        judge::{JudgeRequest, JudgeService},
        judge_output::{ExecutionJobResult, ExecutionResult, JudgeStatus},
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
    UR: UserRepository + Send + Sync + 'static,
    LR: LanguageRepository + Send + Sync + 'static,
    DNR: DepNameRepository<i64> + Send + Sync + 'static,
    JS: JudgeService + Send + Sync + 'static,
> {
    session_repository: SeR,
    submission_repository: SuR,
    problem_repository: PR,
    procedure_repository: PcR,
    testcase_repository: TR,
    user_repository: UR,
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
    UR: UserRepository + Send + Sync + 'static,
    LR: LanguageRepository + Send + Sync + 'static,
    DNR: DepNameRepository<i64> + Send + Sync + 'static,
    JS: JudgeService + Send + Sync + 'static,
> SubmissionService<SeR, SuR, PR, PcR, TR, UR, LR, DNR, JS>
{
    pub fn new(
        session_repository: SeR,
        submission_repository: SuR,
        problem_repository: PR,
        procedure_repository: PcR,
        testcase_repository: TR,
        user_repository: UR,
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
            user_repository,
            language_repository,
            dep_name_repository,
            judge_service,
        }
    }
}

#[derive(Debug)]
pub enum SubmissionError {
    ValidateError,
    Forbidden,
    NotFound,
    InternalServerError,
}

impl<
    SeR: SessionRepository + Send + Sync + 'static,
    SuR: SubmissionRepository + Send + Sync + 'static,
    PR: ProblemRepository + Send + Sync + 'static,
    PcR: ProcedureRepository + Send + Sync + 'static,
    TR: TestcaseRepository + Send + Sync + 'static,
    UR: UserRepository + Send + Sync + 'static,
    LR: LanguageRepository + Send + Sync + 'static,
    DNR: DepNameRepository<i64> + Send + Sync + 'static,
    JS: JudgeService + Send + Sync + 'static,
> SubmissionService<SeR, SuR, PR, PcR, TR, UR, LR, DNR, JS>
{
    pub async fn get_submission(
        &self,
        session_id: Option<&str>,
        submission_id: String,
    ) -> anyhow::Result<SubmissionDto, SubmissionError> {
        let submission_id =
            Uuid::parse_str(&submission_id).map_err(|_| SubmissionError::ValidateError)?;

        let submission = self
            .submission_repository
            .get_submission(submission_id)
            .await
            .map_err(|_| SubmissionError::InternalServerError)?
            .ok_or(SubmissionError::NotFound)?;

        let problem = self
            .problem_repository
            .get_problem(submission.problem_id)
            .await
            .map_err(|_| SubmissionError::InternalServerError)?
            .ok_or(SubmissionError::NotFound)?;

        if !problem.is_public {
            let session_id = session_id.ok_or(SubmissionError::NotFound)?;

            let display_id = self
                .session_repository
                .get_display_id_by_session_id(&session_id)
                .await
                .map_err(|_| SubmissionError::InternalServerError)?
                .ok_or(SubmissionError::NotFound)?;

            if display_id != problem.author_id {
                return Err(SubmissionError::NotFound);
            }
        }

        let judge_results = self
            .submission_repository
            .get_submission_results(submission_id)
            .await
            .map_err(|_| SubmissionError::InternalServerError)?;

        Ok(SubmissionDto {
            id: submission.id.to_string(),
            user_id: submission.user_id.to_string(),
            user_name: submission.user_name,
            problem_id: submission.problem_id.to_string(),
            problem_title: submission.problem_title,
            submitted_at: submission.submitted_at,
            language_id: submission.language_id.to_string(),
            total_score: submission.total_score,
            max_time: submission.max_time,
            max_memory: submission.max_memory,
            code_length: submission.source.len() as i32,
            overall_judge_status: submission.overall_judge_status,
            judge_results: judge_results
                .into_iter()
                .map(|testcase| JudgeResultDto {
                    testcase_id: testcase.testcase_id.to_string(),
                    testcase_name: testcase.testcase_name,
                    judge_status: testcase.judge_status,
                    score: testcase.score,
                    time: testcase.time,
                    memory: testcase.memory,
                })
                .collect(),
        })
    }

    pub async fn get_submissions(
        &self,
        session_id: Option<&str>,
        query: SubmissionGetQueryData,
    ) -> anyhow::Result<SubmissionsDto, SubmissionError> {
        let user_id = match session_id {
            Some(session_id) => self
                .session_repository
                .get_display_id_by_session_id(&session_id)
                .await
                .map_err(|_| SubmissionError::InternalServerError)?,
            None => None,
        };

        let language_id = query.language_id.map_or(Ok(None), |lang_id_str| {
            let lang_id: i64 = lang_id_str
                .parse()
                .map_err(|_| SubmissionError::ValidateError)?;
            Ok(Some(lang_id))
        })?;

        let problem_id = query.problem_id.map_or(Ok(None), |prob_id_str| {
            let prob_id: i64 = prob_id_str
                .parse()
                .map_err(|_| SubmissionError::ValidateError)?;
            Ok(Some(prob_id))
        })?;

        let user_query = query.user_query.map_or(Ok(None), |user_id_str| {
            let user_id: i64 = user_id_str
                .parse()
                .map_err(|_| SubmissionError::ValidateError)?;
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
            .map_err(|_| SubmissionError::InternalServerError)?;

        let submissions = self
            .submission_repository
            .get_submissions_by_query(query)
            .await
            .map_err(|_| SubmissionError::InternalServerError)?;

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
    ) -> anyhow::Result<SubmissionDto, SubmissionError> {
        let problem_id: i64 = problem_id
            .parse()
            .map_err(|_| SubmissionError::ValidateError)?;

        let display_id = match session_id {
            Some(session_id) => self
                .session_repository
                .get_display_id_by_session_id(session_id)
                .await
                .map_err(|_| SubmissionError::InternalServerError)?
                .ok_or(SubmissionError::Forbidden)?,
            None => return Err(SubmissionError::Forbidden),
        };

        let user = self
            .user_repository
            .get_user_by_display_id(display_id)
            .await
            .map_err(|_| SubmissionError::InternalServerError)?
            .ok_or(SubmissionError::Forbidden)?;

        let problem = self
            .problem_repository
            .get_problem(problem_id)
            .await
            .map_err(|_| SubmissionError::InternalServerError)?
            .ok_or(SubmissionError::NotFound)?;

        if !problem.is_public && problem.author_id != display_id {
            return Err(SubmissionError::NotFound);
        }

        let language_id: i32 = body
            .language_id
            .parse()
            .map_err(|_| SubmissionError::ValidateError)?;

        let language = self
            .language_repository
            .id_to_language(language_id)
            .await
            .map_err(|_| SubmissionError::InternalServerError)?
            .ok_or(SubmissionError::ValidateError)?;

        let procedure = self
            .procedure_repository
            .get_procedure(problem_id)
            .await
            .map_err(|_| SubmissionError::InternalServerError)?
            .ok_or(SubmissionError::InternalServerError)?;

        let submission = CreateSubmission {
            problem_id,
            user_id: display_id,
            user_name: user.name.clone(),
            language_id: language_id,
            source: body.source.clone(),
            judge_status: "WJ".to_string(),
            total_score: 0,
            max_time: 0,
            max_memory: 0,
        };

        let submission_id = self
            .submission_repository
            .create_submission(submission)
            .await
            .map_err(|_| SubmissionError::InternalServerError)?;

        let mut runtime_texts = HashMap::new();
        runtime_texts.insert(
            single_judge::SUBMISSION_SOURCE.to_string(),
            body.source.clone(),
        );
        runtime_texts.insert(single_judge::LANGUAGE_TAG.to_string(), language.clone());
        runtime_texts.insert(
            single_judge::TIME_LIMIT_MS.to_string(),
            problem.time_limit.to_string(),
        );
        runtime_texts.insert(
            single_judge::MEMORY_LIMIT_KIB.to_string(),
            (problem.memory_limit as i64 * 1024).to_string(),
        );

        let self_clone = std::sync::Arc::clone(self);

        tokio::spawn(async move {
            let _ = self_clone
                .async_judge_submission(submission_id, problem_id, procedure, runtime_texts)
                .await;
        });

        self.get_submission(session_id, submission_id.to_string())
            .await
    }

    async fn async_judge_submission(
        &self,
        submission_id: Uuid,
        problem_id: i64,
        procedure: judge_core::model::procedure::registered::Procedure,
        runtime_texts: HashMap<String, String>,
    ) -> anyhow::Result<(), SubmissionError> {
        let judge_response = self
            .judge_service
            .judge(JudgeRequest {
                procedure,
                runtime_texts,
            })
            .await
            .map_err(|_| SubmissionError::InternalServerError)?;

        let keys = judge_response.keys().cloned().collect::<Vec<_>>();
        let testcase_names = self
            .dep_name_repository
            .get_many(keys)
            .await
            .map_err(|_| SubmissionError::InternalServerError)?;

        let testcases = self
            .testcase_repository
            .get_testcases(problem_id)
            .await
            .map_err(|_| SubmissionError::InternalServerError)?;

        let name_to_id = testcases
            .into_iter()
            .map(|tc| (tc.name, tc.id))
            .collect::<HashMap<_, _>>();

        let mut total_score: i64 = 0;
        let mut max_time: i32 = 0;
        let mut max_memory: i32 = 0;
        let mut overall_status = JudgeStatus::AC;
        let mut early_exited = false;
        let mut testcase_results: Vec<CreateJudgeResult> = Vec::new();

        for (dep_id, result) in judge_response.into_iter() {
            match result {
                ExecutionJobResult::ExecutionResult(exec) => match exec {
                    ExecutionResult::Displayable(res) => {
                        total_score += res.score;
                        max_time = max_time.max(res.time as i32);
                        max_memory = max_memory.max(res.memory as i32);
                        overall_status = overall_status.max(res.status.clone());

                        let testcase_name = testcase_names
                            .get(&dep_id)
                            .cloned()
                            .flatten()
                            .unwrap_or_default();

                        let testcase_id =
                            name_to_id.get(&testcase_name).cloned().unwrap_or_default();

                        testcase_results.push(CreateJudgeResult {
                            submission_id,
                            testcase_id,
                            testcase_name,
                            judge_status: format!("{:?}", res.status),
                            score: res.score,
                            time: res.time as i32,
                            memory: res.memory as i32,
                        });
                    }
                    ExecutionResult::Hidden(_res) => {
                        // todo
                    }
                },
                ExecutionJobResult::EarlyExit => early_exited = true,
            }
        }

        let overall_status = if early_exited {
            "IE".to_string()
        } else {
            format!("{:?}", overall_status)
        };

        self.submission_repository
            .update_submission(
                submission_id,
                UpdateSubmission {
                    total_score,
                    max_time,
                    max_memory,
                    judge_status: overall_status,
                },
            )
            .await
            .map_err(|_| SubmissionError::InternalServerError)?;

        self.submission_repository
            .create_judge_results(testcase_results)
            .await
            .map_err(|_| SubmissionError::InternalServerError)?;

        Ok(())
    }
}
