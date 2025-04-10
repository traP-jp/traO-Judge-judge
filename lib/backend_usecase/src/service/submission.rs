use crate::model::submission::{
    JudgeResultDto, SubmissionDto, SubmissionGetQueryData, SubmissionOrderByData,
    SubmissionSummaryDto, SubmissionsDto,
};
use domain::{
    model::submission::{SubmissionGetQuery, SubmissionOrderBy},
    repository::{
        problem::ProblemRepository, session::SessionRepository, submission::SubmissionRepository,
    },
};

#[derive(Clone)]
pub struct SubmissionService<
    SeR: SessionRepository,
    SuR: SubmissionRepository,
    PR: ProblemRepository,
> {
    session_repository: SeR,
    submission_repository: SuR,
    problem_repository: PR,
}

impl<SeR: SessionRepository, SuR: SubmissionRepository, PR: ProblemRepository>
    SubmissionService<SeR, SuR, PR>
{
    pub fn new(
        session_repository: SeR,
        submission_repository: SuR,
        problem_repository: PR,
    ) -> Self {
        Self {
            session_repository,
            submission_repository,
            problem_repository,
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

impl<SeR: SessionRepository, SuR: SubmissionRepository, PR: ProblemRepository>
    SubmissionService<SeR, SuR, PR>
{
    pub async fn get_submission(
        &self,
        session_id: Option<String>,
        submission_id: i64,
    ) -> anyhow::Result<SubmissionDto, SubmissionError> {
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
            id: submission.id,
            user_id: submission.user_id,
            user_name: submission.user_name,
            problem_id: submission.problem_id,
            submitted_at: submission.submitted_at,
            language_id: submission.language_id,
            total_score: submission.total_score,
            max_time: submission.max_time,
            max_memory: submission.max_memory,
            code_length: submission.source.len() as i32,
            overall_judge_status: submission.overall_judge_status,
            judge_results: judge_results
                .into_iter()
                .map(|testcase| JudgeResultDto {
                    testcase_id: testcase.testcase_id,
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
        session_id: Option<String>,
        query: SubmissionGetQueryData,
    ) -> anyhow::Result<SubmissionsDto, SubmissionError> {
        let user_id = match session_id {
            Some(session_id) => {
                let display_id = self
                    .session_repository
                    .get_display_id_by_session_id(&session_id)
                    .await
                    .map_err(|_| SubmissionError::InternalServerError)?
                    .ok_or(SubmissionError::NotFound)?;

                Some(display_id)
            }
            None => None,
        };

        let query = SubmissionGetQuery {
            user_id: user_id,
            limit: query.limit.unwrap_or(50),
            offset: query.offset.unwrap_or(0),
            judge_status: query.judge_status,
            language_id: query.language_id,
            user_name: query.user_name,
            user_query: query.user_query,
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
            problem_id: query.problem_id,
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
}
