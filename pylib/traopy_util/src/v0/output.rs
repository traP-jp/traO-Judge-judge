use judge_core::model::judge_output;
use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;
use serde::{Deserialize, Serialize};

/// Judge status enum.
#[pyclass(eq, eq_int)]
#[gen_stub_pyclass_enum]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum JudgeStatus {
    AC,
    WA,
    TLE,
    MLE,
    OLE,
    RE,
    CE,
}

/// Create a displayable result.
#[pyfunction(signature = (status, time, memory, score, continue_next, message=None))]
#[gen_stub_pyfunction]
pub fn displayable_result(
    status: JudgeStatus,
    time: f64,
    memory: f64,
    score: i64,
    continue_next: bool,
    message: Option<String>,
) -> String {
    let result = judge_output::DisplayableExecutionResult {
        status: match status {
            JudgeStatus::AC => judge_output::JudgeStatus::AC,
            JudgeStatus::WA => judge_output::JudgeStatus::WA,
            JudgeStatus::TLE => judge_output::JudgeStatus::TLE,
            JudgeStatus::MLE => judge_output::JudgeStatus::MLE,
            JudgeStatus::OLE => judge_output::JudgeStatus::OLE,
            JudgeStatus::RE => judge_output::JudgeStatus::RE,
            JudgeStatus::CE => judge_output::JudgeStatus::CE,
        },
        time,
        memory,
        score,
        message,
        continue_status: if continue_next {
            judge_output::ContinueStatus::Continue
        } else {
            judge_output::ContinueStatus::Stop
        },
    };
    serde_json::to_string(&result).unwrap()
}

/// Create a hidden result.
#[pyfunction]
#[gen_stub_pyfunction]
pub fn hidden_result(continue_next: bool) -> String {
    let result = judge_output::HiddenExecutionResult {
        continue_status: if continue_next {
            judge_output::ContinueStatus::Continue
        } else {
            judge_output::ContinueStatus::Stop
        },
    };
    serde_json::to_string(&result).unwrap()
}
