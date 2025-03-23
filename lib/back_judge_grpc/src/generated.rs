include!("generated/_.rs");

use anyhow::{Error, Result};
use judge_core::model::procedure::registered;
use judge_core::model::*;
use std::collections::HashMap;

impl From<uuid::Uuid> for Uuid {
    fn from(uuid: uuid::Uuid) -> Self {
        let (high, low) = uuid.as_u64_pair();
        Self { high, low }
    }
}

impl From<Uuid> for uuid::Uuid {
    fn from(uuid: Uuid) -> Self {
        uuid::Uuid::from_u64_pair(uuid.high, uuid.low)
    }
}

impl From<registered::RuntimeText> for RuntimeText {
    fn from(runtime_text: registered::RuntimeText) -> Self {
        let uuid: uuid::Uuid = runtime_text.dep_id.into();
        let uuid: Uuid = uuid.into();
        Self {
            label: runtime_text.label,
            dep_id: Some(uuid),
        }
    }
}

impl TryFrom<RuntimeText> for registered::RuntimeText {
    type Error = Error;
    fn try_from(runtime_text: RuntimeText) -> Result<Self> {
        let uuid = runtime_text
            .dep_id
            .ok_or(anyhow::anyhow!("uuid is missing"))?;
        let uuid: uuid::Uuid = uuid.into();
        let uuid: identifiers::DepId = uuid.into();
        Ok(registered::RuntimeText {
            label: runtime_text.label,
            dep_id: uuid,
        })
    }
}

impl From<registered::Text> for Text {
    fn from(text: registered::Text) -> Self {
        let dep_id: uuid::Uuid = text.dep_id.into();
        let dep_id: Uuid = dep_id.into();
        let resource_id: uuid::Uuid = text.resource_id.into();
        let resource_id: Uuid = resource_id.into();
        Self {
            resource_id: Some(resource_id),
            dep_id: Some(dep_id),
        }
    }
}

impl TryFrom<Text> for registered::Text {
    type Error = Error;
    fn try_from(text: Text) -> Result<Self> {
        let resource_id = text
            .resource_id
            .ok_or(anyhow::anyhow!("resource_id is missing"))?;
        let resource_id: uuid::Uuid = resource_id.into();
        let resource_id: identifiers::ResourceId = resource_id.into();
        let dep_id = text.dep_id.ok_or(anyhow::anyhow!("dep_id is missing"))?;
        let dep_id: uuid::Uuid = dep_id.into();
        let dep_id: identifiers::DepId = dep_id.into();
        Ok(registered::Text {
            resource_id,
            dep_id,
        })
    }
}

impl From<registered::EmptyDirectory> for EmptyDirectory {
    fn from(empty_directory: registered::EmptyDirectory) -> Self {
        let uuid: uuid::Uuid = empty_directory.dep_id.into();
        let uuid: Uuid = uuid.into();
        Self { dep_id: Some(uuid) }
    }
}

impl TryFrom<EmptyDirectory> for registered::EmptyDirectory {
    type Error = Error;
    fn try_from(empty_directory: EmptyDirectory) -> Result<Self> {
        let uuid = empty_directory
            .dep_id
            .ok_or(anyhow::anyhow!("uuid is missing"))?;
        let uuid: uuid::Uuid = uuid.into();
        let uuid: identifiers::DepId = uuid.into();
        Ok(registered::EmptyDirectory { dep_id: uuid })
    }
}

impl From<registered::Dependency> for Dependency {
    fn from(dependency: registered::Dependency) -> Self {
        let uuid: uuid::Uuid = dependency.dep_id.into();
        let uuid: Uuid = uuid.into();
        Self {
            dep_id: Some(uuid),
            envvar_name: dependency.envvar_name,
        }
    }
}

impl TryFrom<Dependency> for registered::Dependency {
    type Error = Error;
    fn try_from(dependency: Dependency) -> Result<Self> {
        let uuid = dependency
            .dep_id
            .ok_or(anyhow::anyhow!("uuid is missing"))?;
        let uuid: uuid::Uuid = uuid.into();
        let uuid: identifiers::DepId = uuid.into();
        Ok(registered::Dependency {
            dep_id: uuid,
            envvar_name: dependency.envvar_name,
        })
    }
}

impl From<registered::Execution> for Execution {
    fn from(execution: registered::Execution) -> Self {
        let dep_id: uuid::Uuid = execution.dep_id.into();
        let dep_id: Uuid = dep_id.into();
        let dependencies = execution
            .dependencies
            .into_iter()
            .map(|dep| dep.into())
            .collect::<prost::alloc::vec::Vec<_>>();
        Self {
            dependencies,
            dep_id: Some(dep_id),
            time_reserved_ms: execution.time_reserved_ms,
        }
    }
}

impl TryFrom<Execution> for registered::Execution {
    type Error = Error;
    fn try_from(execution: Execution) -> Result<Self> {
        let dep_id = execution
            .dep_id
            .ok_or(anyhow::anyhow!("dep_id is missing"))?;
        let dep_id: uuid::Uuid = dep_id.into();
        let dep_id: identifiers::DepId = dep_id.into();
        let dependencies = execution
            .dependencies
            .into_iter()
            .map(|dep| dep.try_into())
            .collect::<Result<prost::alloc::vec::Vec<_>>>()?;
        Ok(registered::Execution {
            dependencies,
            dep_id,
            time_reserved_ms: execution.time_reserved_ms,
        })
    }
}

impl From<registered::Procedure> for Procedure {
    fn from(procedure: registered::Procedure) -> Self {
        let runtime_texts = procedure
            .runtime_texts
            .into_iter()
            .map(|runtime_text| runtime_text.into())
            .collect::<prost::alloc::vec::Vec<_>>();
        let texts = procedure
            .texts
            .into_iter()
            .map(|text| text.into())
            .collect::<prost::alloc::vec::Vec<_>>();
        let empty_directories = procedure
            .empty_directories
            .into_iter()
            .map(|empty_directory| empty_directory.into())
            .collect::<prost::alloc::vec::Vec<_>>();
        let executions = procedure
            .executions
            .into_iter()
            .map(|execution| execution.into())
            .collect::<prost::alloc::vec::Vec<_>>();
        Self {
            runtime_texts,
            texts,
            empty_directories,
            executions,
        }
    }
}

impl TryFrom<Procedure> for registered::Procedure {
    type Error = Error;
    fn try_from(procedure: Procedure) -> Result<Self> {
        let runtime_texts = procedure
            .runtime_texts
            .into_iter()
            .map(|runtime_text| runtime_text.try_into())
            .collect::<Result<prost::alloc::vec::Vec<_>>>()?;
        let texts = procedure
            .texts
            .into_iter()
            .map(|text| text.try_into())
            .collect::<Result<prost::alloc::vec::Vec<_>>>()?;
        let empty_directories = procedure
            .empty_directories
            .into_iter()
            .map(|empty_directory| empty_directory.try_into())
            .collect::<Result<prost::alloc::vec::Vec<_>>>()?;
        let executions = procedure
            .executions
            .into_iter()
            .map(|execution| execution.try_into())
            .collect::<Result<prost::alloc::vec::Vec<_>>>()?;
        Ok(registered::Procedure {
            runtime_texts,
            texts,
            empty_directories,
            executions,
        })
    }
}

impl From<judge::JudgeRequest> for JudgeRequest {
    fn from(judge_request: judge::JudgeRequest) -> Self {
        let procedure: registered::Procedure = judge_request.procedure.into();
        let procedure: Procedure = procedure.into();
        let runtime_text_contents = judge_request
            .runtime_texts
            .into_iter()
            .map(|(k, v)| RuntimeTextContent {
                label: k,
                content: v,
            })
            .collect::<prost::alloc::vec::Vec<_>>();
        Self {
            procedure: Some(procedure),
            runtime_text_contents,
        }
    }
}

impl TryFrom<JudgeRequest> for judge::JudgeRequest {
    type Error = Error;
    fn try_from(judge_request: JudgeRequest) -> Result<Self> {
        let procedure = judge_request
            .procedure
            .ok_or(anyhow::anyhow!("procedure is missing"))?;
        let procedure: Result<registered::Procedure> = procedure.try_into();
        let procedure = procedure.map_err(|err| anyhow::anyhow!("procedure: {}", err))?;
        let runtime_texts = judge_request
            .runtime_text_contents
            .into_iter()
            .map(|content| (content.label, content.content))
            .collect::<HashMap<String, String>>();
        Ok(judge::JudgeRequest {
            procedure,
            runtime_texts,
        })
    }
}

impl From<judge_output::JudgeStatus> for JudgeStatus {
    fn from(judge_status: judge_output::JudgeStatus) -> Self {
        match judge_status {
            judge_output::JudgeStatus::AC => Self::Ac,
            judge_output::JudgeStatus::WA => Self::Wa,
            judge_output::JudgeStatus::TLE => Self::Tle,
            judge_output::JudgeStatus::RE => Self::Re,
            judge_output::JudgeStatus::CE => Self::Ce,
            judge_output::JudgeStatus::MLE => Self::Mle,
            judge_output::JudgeStatus::OLE => Self::Ole,
        }
    }
}

impl From<JudgeStatus> for judge_output::JudgeStatus {
    fn from(judge_status: JudgeStatus) -> Self {
        match judge_status {
            JudgeStatus::Ac => Self::AC,
            JudgeStatus::Wa => Self::WA,
            JudgeStatus::Tle => Self::TLE,
            JudgeStatus::Re => Self::RE,
            JudgeStatus::Ce => Self::CE,
            JudgeStatus::Mle => Self::MLE,
            JudgeStatus::Ole => Self::OLE,
        }
    }
}

impl From<judge_output::ContinueStatus> for ContinueStatus {
    fn from(continue_status: judge_output::ContinueStatus) -> Self {
        match continue_status {
            judge_output::ContinueStatus::Continue => Self::Continue,
            judge_output::ContinueStatus::Stop => Self::Stop,
        }
    }
}

impl From<ContinueStatus> for judge_output::ContinueStatus {
    fn from(continue_status: ContinueStatus) -> Self {
        match continue_status {
            ContinueStatus::Continue => Self::Continue,
            ContinueStatus::Stop => Self::Stop,
        }
    }
}

impl From<judge_output::DisplayableExecutionResult> for DisplayableExecutionResult {
    fn from(displayable_execution_result: judge_output::DisplayableExecutionResult) -> Self {
        let stauts: JudgeStatus = displayable_execution_result.status.into();
        let status: i32 = stauts.into();
        let continue_status: ContinueStatus = displayable_execution_result.continue_status.into();
        let continue_status: i32 = continue_status.into();
        Self {
            status,
            execution_time: displayable_execution_result.time,
            used_memory: displayable_execution_result.memory,
            score: displayable_execution_result.score,
            message: displayable_execution_result.message,
            continue_status,
        }
    }
}

impl TryFrom<DisplayableExecutionResult> for judge_output::DisplayableExecutionResult {
    type Error = Error;
    fn try_from(displayable_execution_result: DisplayableExecutionResult) -> Result<Self> {
        let status = JudgeStatus::try_from(displayable_execution_result.status)?;
        let status: judge_output::JudgeStatus = status.into();
        let continue_status =
            ContinueStatus::try_from(displayable_execution_result.continue_status)?;
        let continue_status: judge_output::ContinueStatus = continue_status.into();
        Ok(judge_output::DisplayableExecutionResult {
            status,
            time: displayable_execution_result.execution_time,
            memory: displayable_execution_result.used_memory,
            score: displayable_execution_result.score,
            message: displayable_execution_result.message,
            continue_status,
        })
    }
}

impl From<judge_output::HiddenExecutionResult> for HiddenExecutionResult {
    fn from(hidden_execution_result: judge_output::HiddenExecutionResult) -> Self {
        let continue_status: ContinueStatus = hidden_execution_result.continue_status.into();
        let continue_status: i32 = continue_status.into();
        Self { continue_status }
    }
}

impl TryFrom<HiddenExecutionResult> for judge_output::HiddenExecutionResult {
    type Error = Error;
    fn try_from(hidden_execution_result: HiddenExecutionResult) -> Result<Self> {
        let continue_status = ContinueStatus::try_from(hidden_execution_result.continue_status)?;
        let continue_status: judge_output::ContinueStatus = continue_status.into();
        Ok(judge_output::HiddenExecutionResult { continue_status })
    }
}

impl From<judge_output::ExecutionJobResult> for ExecutionJobResult {
    fn from(execution_job_result: judge_output::ExecutionJobResult) -> Self {
        let result = match execution_job_result {
            judge_output::ExecutionJobResult::ExecutionResult(execution_result) => {
                match execution_result {
                    judge_output::ExecutionResult::Displayable(displayable) => {
                        let displayable: DisplayableExecutionResult = displayable.into();
                        execution_job_result::Result::DisplayableExecutionResult(displayable)
                    }
                    judge_output::ExecutionResult::Hidden(hidden) => {
                        let hidden: HiddenExecutionResult = hidden.into();
                        execution_job_result::Result::HiddenExecutionResult(hidden)
                    }
                }
            }
            judge_output::ExecutionJobResult::EarlyExit => {
                execution_job_result::Result::EarlyReturn(Unit {})
            }
        };
        Self {
            result: Some(result),
        }
    }
}

impl TryFrom<ExecutionJobResult> for judge_output::ExecutionJobResult {
    type Error = Error;
    fn try_from(execution_job_result: ExecutionJobResult) -> Result<Self> {
        let result = execution_job_result
            .result
            .ok_or(anyhow::anyhow!("result is missing"))?;
        match result {
            execution_job_result::Result::DisplayableExecutionResult(displayable) => {
                let displayable: judge_output::DisplayableExecutionResult =
                    displayable.try_into()?;
                Ok(judge_output::ExecutionJobResult::ExecutionResult(
                    judge_output::ExecutionResult::Displayable(displayable),
                ))
            }
            execution_job_result::Result::HiddenExecutionResult(hidden) => {
                let hidden: judge_output::HiddenExecutionResult = hidden.try_into()?;
                Ok(judge_output::ExecutionJobResult::ExecutionResult(
                    judge_output::ExecutionResult::Hidden(hidden),
                ))
            }
            execution_job_result::Result::EarlyReturn(_) => {
                Ok(judge_output::ExecutionJobResult::EarlyExit)
            }
        }
    }
}

impl From<judge::JudgeResponse> for JudgeResponse {
    fn from(judge_response: judge::JudgeResponse) -> Self {
        match judge_response {
            Ok(results) => {
                let results = results
                    .into_iter()
                    .map(|(k, v)| {
                        let uuid: uuid::Uuid = k.into();
                        let uuid: Uuid = uuid.into();
                        let result: ExecutionJobResult = v.into();
                        let result_with_id = ExecutionJobResultWithDepId {
                            execution_job_result: Some(result),
                            dep_id: Some(uuid),
                        };
                        result_with_id
                    })
                    .collect::<prost::alloc::vec::Vec<_>>();
                let results = ExecutionJobResults {
                    execution_job_results: results,
                };
                let results = judge_response::Result::ExecutionJobResults(results);
                Self {
                    result: Some(results),
                }
            }
            Err(err) => Self {
                result: Some(judge_response::Result::ErrorMessage(err.to_string())),
            },
        }
    }
}

impl From<JudgeResponse> for judge::JudgeResponse {
    fn from(judge_response: JudgeResponse) -> Self {
        let results = judge_response
            .result
            .ok_or(anyhow::anyhow!("result is missing"))?;
        match results {
            judge_response::Result::ExecutionJobResults(results) => {
                let results = results.execution_job_results
                    .into_iter()
                    .map(|result| {
                        let dep_id = result.dep_id.ok_or(anyhow::anyhow!("dep_id is missing"))?;
                        let dep_id: uuid::Uuid = dep_id.into();
                        let dep_id: identifiers::DepId = dep_id.into();
                        let result = result.execution_job_result.ok_or(anyhow::anyhow!("execution_job_result is missing"))?;
                        let result: judge_output::ExecutionJobResult = result.try_into()?;
                        Ok((dep_id, result))
                    })
                    .collect::<Result<HashMap<identifiers::DepId, judge_output::ExecutionJobResult>>>()?;
                Ok(results)
            }
            judge_response::Result::ErrorMessage(err) => Err(anyhow::anyhow!(err)),
        }
    }
}
