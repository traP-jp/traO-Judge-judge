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

/// Judge status enum.
#[pyclass]
#[gen_stub_pyclass]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct JudgeStatusPriority {
    pub ac: i32,
    pub wa: i32,
    pub tle: i32,
    pub mle: i32,
    pub ole: i32,
    pub re: i32,
    pub ce: i32,
}

const DEFAULT_PRIORITY: JudgeStatusPriority = JudgeStatusPriority {
    ac: 0,
    wa: 3,
    tle: 2,
    mle: 1,
    ole: 4,
    re: 5,
    ce: 6,
};

/// Merge judge status.
#[pyfunction(signature = (status_vec, priority=None))]
#[gen_stub_pyfunction]
pub fn merge_judge_status(
    status_vec: Vec<JudgeStatus>,
    priority: Option<JudgeStatusPriority>,
) -> PyResult<JudgeStatus> {
    let priority = priority.unwrap_or(DEFAULT_PRIORITY);
    let max_status = status_vec
        .into_iter()
        .max_by_key(|status| match status {
            JudgeStatus::AC => priority.ac,
            JudgeStatus::WA => priority.wa,
            JudgeStatus::TLE => priority.tle,
            JudgeStatus::MLE => priority.mle,
            JudgeStatus::OLE => priority.ole,
            JudgeStatus::RE => priority.re,
            JudgeStatus::CE => priority.ce,
        })
        .unwrap();
    Ok(max_status)
}
