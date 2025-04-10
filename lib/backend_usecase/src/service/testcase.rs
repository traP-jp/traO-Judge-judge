use domain::repository::{
    problem::ProblemRepository, procedure::ProcedureRepository, session::SessionRepository,
    testcase::TestcaseRepository,
};
use judge_core::model::{
    dep_name_repository::DepNameRepository,
    problem_registry::{ProblemRegistryClient, ProblemRegistryServer},
};

use crate::model::testcase::{TestcaseDto, TestcaseSummaryDto};

#[derive(Clone)]
pub struct TestcaseService<
    PR: ProblemRepository,
    SR: SessionRepository,
    TR: TestcaseRepository,
    PcR: ProcedureRepository,
    RPC: ProblemRegistryClient,
    PRS: ProblemRegistryServer,
    DNR: DepNameRepository<i64>,
> {
    problem_repository: PR,
    session_repository: SR,
    testcase_repository: TR,
    procedure_repository: PcR,
    problem_registry_client: RPC,
    problem_registry_server: PRS,
    dep_name_repository: DNR,
}

impl<
        PR: ProblemRepository,
        SR: SessionRepository,
        TR: TestcaseRepository,
        PcR: ProcedureRepository,
        RPC: ProblemRegistryClient,
        PRS: ProblemRegistryServer,
        DNR: DepNameRepository<i64>,
    > TestcaseService<PR, SR, TR, PcR, RPC, PRS, DNR>
{
    pub fn new(
        problem_repository: PR,
        session_repository: SR,
        testcase_repository: TR,
        procedure_repository: PcR,
        problem_registry_client: RPC,
        problem_registry_server: PRS,
        dep_name_repository: DNR,
    ) -> Self {
        Self {
            problem_repository,
            session_repository,
            testcase_repository,
            procedure_repository,
            problem_registry_client,
            problem_registry_server,
            dep_name_repository,
        }
    }
}

#[derive(Debug)]
pub enum TestcaseError {
    ValidateError,
    Unauthorized,
    Forbidden,
    NotFound,
    InternalServerError,
}

impl<
        PR: ProblemRepository,
        SR: SessionRepository,
        TR: TestcaseRepository,
        PcR: ProcedureRepository,
        RPC: ProblemRegistryClient,
        PRS: ProblemRegistryServer,
        DNR: DepNameRepository<i64>,
    > TestcaseService<PR, SR, TR, PcR, RPC, PRS, DNR>
{
    pub async fn get_testcases(
        &self,
        session_id: Option<String>,
        problem_id: i64,
    ) -> Result<Vec<TestcaseSummaryDto>, TestcaseError> {
        let problem = self
            .problem_repository
            .get_problem(problem_id)
            .await
            .map_err(|_| TestcaseError::InternalServerError)?;

        match problem {
            Some(problem) => {
                if !problem.is_public {
                    let session_id = session_id.ok_or(TestcaseError::NotFound)?;

                    let user_id = self
                        .session_repository
                        .get_display_id_by_session_id(&session_id)
                        .await
                        .map_err(|_| TestcaseError::InternalServerError)?
                        .ok_or(TestcaseError::NotFound)?;

                    if problem.author_id != user_id {
                        return Err(TestcaseError::NotFound);
                    }
                }
            }
            None => return Err(TestcaseError::NotFound),
        }

        let testcases = self
            .testcase_repository
            .get_testcases(problem_id)
            .await
            .map_err(|_| TestcaseError::InternalServerError)?;

        Ok(testcases.into_iter().map(|x| x.into()).collect())
    }

    pub async fn get_testcase(
        &self,
        session_id: Option<String>,
        testcase_id: i64,
    ) -> Result<TestcaseDto, TestcaseError> {
        let testcase = self
            .testcase_repository
            .get_testcase(testcase_id)
            .await
            .map_err(|_| TestcaseError::InternalServerError)?
            .ok_or(TestcaseError::NotFound)?;

        let problem = self
            .problem_repository
            .get_problem(testcase.problem_id)
            .await
            .map_err(|_| TestcaseError::InternalServerError)?
            .ok_or(TestcaseError::NotFound)?;

        if !problem.is_public {
            let session_id = session_id.ok_or(TestcaseError::NotFound)?;

            let user_id = self
                .session_repository
                .get_display_id_by_session_id(&session_id)
                .await
                .map_err(|_| TestcaseError::InternalServerError)?
                .ok_or(TestcaseError::NotFound)?;

            if problem.author_id != user_id {
                return Err(TestcaseError::NotFound);
            }
        }

        // todo (testcase中身の取得)
        // let input = self.problem_registry_client.fetch(...)...;
        // let output = self.problem_registry_client.fetch(...)...;

        let testcase = TestcaseDto {
            id: testcase.id,
            name: testcase.name,
            input: "todo".to_string(),  // input,
            output: "todo".to_string(), // output,
            created_at: testcase.created_at,
            updated_at: testcase.updated_at,
        };

        Ok(testcase)
    }
}
