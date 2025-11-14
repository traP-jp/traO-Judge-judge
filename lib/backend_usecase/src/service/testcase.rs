use domain::{
    model::testcase::CreateTestcase,
    repository::{
        problem::ProblemRepository, procedure::ProcedureRepository, session::SessionRepository,
        testcase::TestcaseRepository,
    },
};
use judge_core::{
    constant::job_name::v0_features::{testcase_expected_name, testcase_input_name},
    logic::{
        problem_presets::normal_judge::{NormalJudgeTestcase, create_normal_judge_procedure},
        writer_schema_registerer::register,
    },
    model::{
        dep_name_repository::DepNameRepository,
        problem_registry::{ProblemRegistryClient, ProblemRegistryServer},
    },
};
use uuid::Uuid;

use crate::model::{
    error::UsecaseError,
    testcase::{CreateTestcaseData, TestcaseDto, TestcaseSummaryDto, UpdateTestcaseData},
};

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
        session_id: Option<&str>,
        problem_id: String,
    ) -> Result<Vec<TestcaseSummaryDto>, UsecaseError> {
        let problem_id = problem_id
            .parse::<i64>()
            .map_err(|_| UsecaseError::ValidateError)?;

        let problem = self
            .problem_repository
            .get_problem(problem_id)
            .await
            .map_err(UsecaseError::internal_server_error)?;

        match problem {
            Some(problem) => {
                if !problem.is_public {
                    let session_id = session_id.ok_or(UsecaseError::NotFound)?;

                    let user_id = self
                        .session_repository
                        .get_display_id_by_session_id(session_id)
                        .await
                        .map_err(UsecaseError::internal_server_error)?
                        .ok_or(UsecaseError::NotFound)?;

                    if problem.author_id != user_id {
                        return Err(UsecaseError::NotFound);
                    }
                }
            }
            None => return Err(UsecaseError::NotFound),
        }

        let testcases = self
            .testcase_repository
            .get_testcases(problem_id)
            .await
            .map_err(UsecaseError::internal_server_error)?;

        Ok(testcases.into_iter().map(|x| x.into()).collect())
    }

    pub async fn get_testcase(
        &self,
        session_id: Option<&str>,
        testcase_id: String,
    ) -> Result<TestcaseDto, UsecaseError> {
        let testcase_id = Uuid::parse_str(&testcase_id).map_err(|_| UsecaseError::ValidateError)?;

        let testcase = self
            .testcase_repository
            .get_testcase(testcase_id)
            .await
            .map_err(UsecaseError::internal_server_error)?
            .ok_or(UsecaseError::NotFound)?;

        let problem = self
            .problem_repository
            .get_problem(testcase.problem_id)
            .await
            .map_err(UsecaseError::internal_server_error)?
            .ok_or(UsecaseError::NotFound)?;

        if !problem.is_public {
            let session_id = session_id.ok_or(UsecaseError::NotFound)?;

            let user_id = self
                .session_repository
                .get_display_id_by_session_id(session_id)
                .await
                .map_err(UsecaseError::internal_server_error)?
                .ok_or(UsecaseError::NotFound)?;

            if problem.author_id != user_id {
                return Err(UsecaseError::NotFound);
            }
        }

        let input = self
            .problem_registry_client
            .fetch(testcase.input_id.into())
            .await
            .map_err(UsecaseError::internal_server_error)?;
        let output = self
            .problem_registry_client
            .fetch(testcase.output_id.into())
            .await
            .map_err(UsecaseError::internal_server_error)?;

        let testcase = TestcaseDto {
            id: testcase.id,
            name: testcase.name,
            input,
            output,
            created_at: testcase.created_at,
            updated_at: testcase.updated_at,
        };

        Ok(testcase)
    }

    pub async fn post_testcases(
        &self,
        session_id: Option<&str>,
        problem_id: String,
        testcases: Vec<CreateTestcaseData>,
    ) -> Result<(), UsecaseError> {
        let problem_id = problem_id
            .parse::<i64>()
            .map_err(|_| UsecaseError::ValidateError)?;

        let problem = self
            .problem_repository
            .get_problem(problem_id)
            .await
            .map_err(UsecaseError::internal_server_error)?
            .ok_or(UsecaseError::NotFound)?;

        let user_id = match session_id {
            Some(session_id) => self
                .session_repository
                .get_display_id_by_session_id(session_id)
                .await
                .map_err(UsecaseError::internal_server_error)?,
            None => None,
        };

        if !problem.is_public && user_id.is_none_or(|x| x != problem.author_id) {
            return Err(UsecaseError::NotFound);
        }

        if user_id.is_none_or(|x| x != problem.author_id) {
            return Err(UsecaseError::Forbidden);
        }

        let now_testcases = self
            .testcase_repository
            .get_testcases(problem_id)
            .await
            .map_err(UsecaseError::internal_server_error)?;

        // now_testcasesとtestcasesの和集合で名前被りがないか確認(now_testcasesの名前は重複しない)
        {
            let mut name_set = std::collections::HashSet::new();
            for testcase in now_testcases.iter() {
                name_set.insert(testcase.name.clone());
            }
            for testcase in testcases.iter() {
                if name_set.contains(&testcase.name) {
                    return Err(UsecaseError::ValidateError);
                }
                name_set.insert(testcase.name.clone());
            }
        }

        // procedureを作成し、保存
        let mut new_testcases = Vec::new();
        for testcase in now_testcases.iter() {
            let input = self
                .problem_registry_client
                .fetch(testcase.input_id.into())
                .await
                .map_err(UsecaseError::internal_server_error)?;

            let output = self
                .problem_registry_client
                .fetch(testcase.output_id.into())
                .await
                .map_err(UsecaseError::internal_server_error)?;

            new_testcases.push(NormalJudgeTestcase {
                name: testcase.name.clone(),
                input,
                expected_output: output,
            });
        }
        for testcase in testcases.iter() {
            new_testcases.push(NormalJudgeTestcase {
                name: testcase.name.clone(),
                input: testcase.input.clone(),
                expected_output: testcase.output.clone(),
            });
        }

        let procedure = create_normal_judge_procedure(new_testcases)
            .map_err(UsecaseError::internal_server_error)?;

        self.dep_name_repository
            .remove_many(problem_id)
            .await
            .map_err(UsecaseError::internal_server_error)?;
        let registered_procedure = register(
            procedure,
            self.problem_registry_server.clone(),
            self.dep_name_repository.clone(),
            problem_id,
        )
        .await
        .map_err(UsecaseError::internal_server_error)?;

        let dep_id_to_resource_id = {
            let mut dep_id_to_resource_id = std::collections::HashMap::new();
            for text in registered_procedure.texts.iter() {
                dep_id_to_resource_id.insert(text.dep_id, text.resource_id);
            }
            dep_id_to_resource_id
        };

        self.procedure_repository
            .update_procedure(problem_id, registered_procedure)
            .await
            .map_err(UsecaseError::internal_server_error)?;

        // データベースに保存するためのtestcasesを作成
        let name_to_id = {
            let id_to_name = self
                .dep_name_repository
                .get_many_by_problem_id(problem_id)
                .await
                .map_err(UsecaseError::internal_server_error)?;
            let mut name_to_id = std::collections::HashMap::new();
            for (id, name) in id_to_name {
                name_to_id.insert(name, id);
            }
            name_to_id
        };

        let mut new_testcases: Vec<CreateTestcase> = Vec::new();
        for testcase in now_testcases.iter() {
            let input_id = name_to_id
                .get(testcase_input_name(&testcase.name).as_str())
                .ok_or_else(|| {
                    UsecaseError::internal_server_error_msg("missing dep name for testcase input")
                })?;
            let input_id = dep_id_to_resource_id.get(input_id).ok_or_else(|| {
                UsecaseError::internal_server_error_msg(
                    "missing resource id for testcase input dep",
                )
            })?;

            let output_id = name_to_id
                .get(testcase_expected_name(&testcase.name).as_str())
                .ok_or_else(|| {
                    UsecaseError::internal_server_error_msg(
                        "missing dep name for testcase expected output",
                    )
                })?;
            let output_id = dep_id_to_resource_id.get(output_id).ok_or_else(|| {
                UsecaseError::internal_server_error_msg(
                    "missing resource id for testcase expected output dep",
                )
            })?;

            new_testcases.push(CreateTestcase {
                id: testcase.id,
                problem_id,
                name: testcase.name.clone(),
                input_id: input_id.to_owned().into(),
                output_id: output_id.to_owned().into(),
            });
        }
        for testcase in testcases {
            let input_id = name_to_id
                .get(testcase_input_name(&testcase.name).as_str())
                .ok_or_else(|| {
                    UsecaseError::internal_server_error_msg(
                        "missing dep name for testcase input (new)",
                    )
                })?;
            let input_id = dep_id_to_resource_id.get(input_id).ok_or_else(|| {
                UsecaseError::internal_server_error_msg(
                    "missing resource id for testcase input dep (new)",
                )
            })?;
            let output_id = name_to_id
                .get(testcase_expected_name(&testcase.name).as_str())
                .ok_or_else(|| {
                    UsecaseError::internal_server_error_msg(
                        "missing dep name for testcase expected output (new)",
                    )
                })?;
            let output_id = dep_id_to_resource_id.get(output_id).ok_or_else(|| {
                UsecaseError::internal_server_error_msg(
                    "missing resource id for testcase expected output dep (new)",
                )
            })?;

            new_testcases.push(CreateTestcase {
                id: Uuid::now_v7(),
                problem_id,
                name: testcase.name,
                input_id: input_id.to_owned().into(),
                output_id: output_id.to_owned().into(),
            });
        }

        // testcasesの更新
        self.testcase_repository
            .delete_testcases(problem_id)
            .await
            .map_err(UsecaseError::internal_server_error)?;
        self.testcase_repository
            .create_testcases(new_testcases)
            .await
            .map_err(UsecaseError::internal_server_error)?;

        Ok(())
    }

    pub async fn delete_testcase(
        &self,
        session_id: Option<&str>,
        testcase_id: String,
    ) -> Result<(), UsecaseError> {
        let testcase_id = Uuid::parse_str(&testcase_id).map_err(|_| UsecaseError::ValidateError)?;

        let testcase = self
            .testcase_repository
            .get_testcase(testcase_id)
            .await
            .map_err(UsecaseError::internal_server_error)?
            .ok_or(UsecaseError::NotFound)?;

        let problem = self
            .problem_repository
            .get_problem(testcase.problem_id)
            .await
            .map_err(UsecaseError::internal_server_error)?
            .ok_or(UsecaseError::NotFound)?;

        let user_id = match session_id {
            Some(session_id) => self
                .session_repository
                .get_display_id_by_session_id(session_id)
                .await
                .map_err(UsecaseError::internal_server_error)?,
            None => None,
        };

        if !problem.is_public && user_id.is_none_or(|x| x != problem.author_id) {
            return Err(UsecaseError::NotFound);
        }

        if user_id.is_none_or(|x| x != problem.author_id) {
            return Err(UsecaseError::Forbidden);
        }

        let now_testcases = self
            .testcase_repository
            .get_testcases(problem.id)
            .await
            .map_err(UsecaseError::internal_server_error)?;

        let mut new_testcases = Vec::new();
        for testcase in now_testcases.iter() {
            if testcase.id != testcase_id {
                let input = self
                    .problem_registry_client
                    .fetch(testcase.input_id.into())
                    .await
                    .map_err(UsecaseError::internal_server_error)?;

                let output = self
                    .problem_registry_client
                    .fetch(testcase.output_id.into())
                    .await
                    .map_err(UsecaseError::internal_server_error)?;

                new_testcases.push(NormalJudgeTestcase {
                    name: testcase.name.clone(),
                    input,
                    expected_output: output,
                });
            }
        }

        let procedure = create_normal_judge_procedure(new_testcases)
            .map_err(UsecaseError::internal_server_error)?;

        self.dep_name_repository
            .remove_many(problem.id)
            .await
            .map_err(UsecaseError::internal_server_error)?;
        let registered_procedure = register(
            procedure,
            self.problem_registry_server.clone(),
            self.dep_name_repository.clone(),
            problem.id,
        )
        .await
        .map_err(UsecaseError::internal_server_error)?;

        let dep_id_to_resource_id = {
            let mut dep_id_to_resource_id = std::collections::HashMap::new();
            for text in registered_procedure.texts.iter() {
                dep_id_to_resource_id.insert(text.dep_id, text.resource_id);
            }
            dep_id_to_resource_id
        };

        self.procedure_repository
            .update_procedure(problem.id, registered_procedure)
            .await
            .map_err(UsecaseError::internal_server_error)?;

        // データベースに保存するためのtestcasesを作成
        let name_to_id = {
            let id_to_name = self
                .dep_name_repository
                .get_many_by_problem_id(problem.id)
                .await
                .map_err(UsecaseError::internal_server_error)?;
            let mut name_to_id = std::collections::HashMap::new();
            for (id, name) in id_to_name {
                name_to_id.insert(name, id);
            }
            name_to_id
        };
        let mut new_testcases: Vec<CreateTestcase> = Vec::new();
        for testcase in now_testcases.iter() {
            if testcase.id != testcase_id {
                let input_id = name_to_id
                    .get(testcase_input_name(&testcase.name).as_str())
                    .ok_or_else(|| {
                        UsecaseError::internal_server_error_msg(
                            "missing dep name for testcase input (existing)",
                        )
                    })?;
                let input_id = dep_id_to_resource_id.get(input_id).ok_or_else(|| {
                    UsecaseError::internal_server_error_msg(
                        "missing resource id for testcase input dep (existing)",
                    )
                })?;

                let output_id = name_to_id
                    .get(testcase_expected_name(&testcase.name).as_str())
                    .ok_or_else(|| {
                        UsecaseError::internal_server_error_msg(
                            "missing dep name for testcase expected output (existing)",
                        )
                    })?;
                let output_id = dep_id_to_resource_id.get(output_id).ok_or_else(|| {
                    UsecaseError::internal_server_error_msg(
                        "missing resource id for testcase expected output dep (existing)",
                    )
                })?;

                new_testcases.push(CreateTestcase {
                    id: testcase.id,
                    problem_id: problem.id,
                    name: testcase.name.clone(),
                    input_id: input_id.to_owned().into(),
                    output_id: output_id.to_owned().into(),
                });
            }
        }

        // testcasesの更新
        self.testcase_repository
            .delete_testcases(problem.id)
            .await
            .map_err(UsecaseError::internal_server_error)?;
        self.testcase_repository
            .create_testcases(new_testcases)
            .await
            .map_err(UsecaseError::internal_server_error)?;

        Ok(())
    }

    pub async fn put_testcase(
        &self,
        session_id: Option<&str>,
        testcase_id: String,
        put_testcase: UpdateTestcaseData,
    ) -> Result<(), UsecaseError> {
        let testcase_id = Uuid::parse_str(&testcase_id).map_err(|_| UsecaseError::ValidateError)?;

        let testcase = self
            .testcase_repository
            .get_testcase(testcase_id)
            .await
            .map_err(UsecaseError::internal_server_error)?
            .ok_or(UsecaseError::NotFound)?;

        let problem = self
            .problem_repository
            .get_problem(testcase.problem_id)
            .await
            .map_err(UsecaseError::internal_server_error)?
            .ok_or(UsecaseError::NotFound)?;

        let user_id = match session_id {
            Some(session_id) => self
                .session_repository
                .get_display_id_by_session_id(session_id)
                .await
                .map_err(UsecaseError::internal_server_error)?,
            None => None,
        };

        if !problem.is_public && user_id.is_none_or(|x| x != problem.author_id) {
            return Err(UsecaseError::NotFound);
        }

        if user_id.is_none_or(|x| x != problem.author_id) {
            return Err(UsecaseError::Forbidden);
        }

        let now_testcases = self
            .testcase_repository
            .get_testcases(problem.id)
            .await
            .map_err(UsecaseError::internal_server_error)?;

        // 自身を除いたtestcasesの名前が一致するか判定
        for testcase in now_testcases.iter() {
            if testcase.id != testcase_id && put_testcase.name == testcase.name {
                return Err(UsecaseError::ValidateError);
            }
        }

        // procedureを作成し、保存
        let mut new_testcases = Vec::new();
        for testcase in now_testcases.iter() {
            if testcase.id != testcase_id {
                let input = self
                    .problem_registry_client
                    .fetch(testcase.input_id.into())
                    .await
                    .map_err(UsecaseError::internal_server_error)?;

                let output = self
                    .problem_registry_client
                    .fetch(testcase.output_id.into())
                    .await
                    .map_err(UsecaseError::internal_server_error)?;

                new_testcases.push(NormalJudgeTestcase {
                    name: testcase.name.clone(),
                    input,
                    expected_output: output,
                });
            } else {
                new_testcases.push(NormalJudgeTestcase {
                    name: put_testcase.name.clone(),
                    input: put_testcase.input.clone(),
                    expected_output: put_testcase.output.clone(),
                });
            }
        }

        let procedure = create_normal_judge_procedure(new_testcases)
            .map_err(UsecaseError::internal_server_error)?;

        self.dep_name_repository
            .remove_many(problem.id)
            .await
            .map_err(UsecaseError::internal_server_error)?;
        let registered_procedure = register(
            procedure,
            self.problem_registry_server.clone(),
            self.dep_name_repository.clone(),
            problem.id,
        )
        .await
        .map_err(UsecaseError::internal_server_error)?;

        let dep_id_to_resource_id = {
            let mut dep_id_to_resource_id = std::collections::HashMap::new();
            for text in registered_procedure.texts.iter() {
                dep_id_to_resource_id.insert(text.dep_id, text.resource_id);
            }
            dep_id_to_resource_id
        };

        self.procedure_repository
            .update_procedure(problem.id, registered_procedure)
            .await
            .map_err(UsecaseError::internal_server_error)?;

        // データベースに保存するためのtestcasesを作成
        let name_to_id = {
            let id_to_name = self
                .dep_name_repository
                .get_many_by_problem_id(problem.id)
                .await
                .map_err(UsecaseError::internal_server_error)?;
            let mut name_to_id = std::collections::HashMap::new();
            for (id, name) in id_to_name {
                name_to_id.insert(name, id);
            }
            name_to_id
        };
        let mut new_testcases: Vec<CreateTestcase> = Vec::new();
        for testcase in now_testcases.iter() {
            if testcase.id != testcase_id {
                let input_id = name_to_id
                    .get(testcase_input_name(&testcase.name).as_str())
                    .ok_or_else(|| {
                        UsecaseError::internal_server_error_msg(
                            "missing dep name for testcase input (edit existing)",
                        )
                    })?;
                let input_id = dep_id_to_resource_id.get(input_id).ok_or_else(|| {
                    UsecaseError::internal_server_error_msg(
                        "missing resource id for testcase input dep (edit existing)",
                    )
                })?;

                let output_id = name_to_id
                    .get(testcase_expected_name(&testcase.name).as_str())
                    .ok_or_else(|| {
                        UsecaseError::internal_server_error_msg(
                            "missing dep name for testcase expected output (edit existing)",
                        )
                    })?;
                let output_id = dep_id_to_resource_id.get(output_id).ok_or_else(|| {
                    UsecaseError::internal_server_error_msg(
                        "missing resource id for testcase expected output dep (edit existing)",
                    )
                })?;

                new_testcases.push(CreateTestcase {
                    id: testcase.id,
                    problem_id: problem.id,
                    name: testcase.name.clone(),
                    input_id: input_id.to_owned().into(),
                    output_id: output_id.to_owned().into(),
                });
            } else {
                let input_id = name_to_id
                    .get(testcase_input_name(&put_testcase.name).as_str())
                    .ok_or_else(|| {
                        UsecaseError::internal_server_error_msg(
                            "missing dep name for testcase input (put)",
                        )
                    })?;
                let input_id = dep_id_to_resource_id.get(input_id).ok_or_else(|| {
                    UsecaseError::internal_server_error_msg(
                        "missing resource id for testcase input dep (put)",
                    )
                })?;

                let output_id = name_to_id
                    .get(testcase_expected_name(&put_testcase.name).as_str())
                    .ok_or_else(|| {
                        UsecaseError::internal_server_error_msg(
                            "missing dep name for testcase expected output (put)",
                        )
                    })?;
                let output_id = dep_id_to_resource_id.get(output_id).ok_or_else(|| {
                    UsecaseError::internal_server_error_msg(
                        "missing resource id for testcase expected output dep (put)",
                    )
                })?;

                new_testcases.push(CreateTestcase {
                    id: testcase.id,
                    problem_id: problem.id,
                    name: put_testcase.name.clone(),
                    input_id: input_id.to_owned().into(),
                    output_id: output_id.to_owned().into(),
                });
            }
        }

        // testcasesの更新
        self.testcase_repository
            .delete_testcases(problem.id)
            .await
            .map_err(UsecaseError::internal_server_error)?;
        self.testcase_repository
            .create_testcases(new_testcases)
            .await
            .map_err(UsecaseError::internal_server_error)?;

        Ok(())
    }
}
