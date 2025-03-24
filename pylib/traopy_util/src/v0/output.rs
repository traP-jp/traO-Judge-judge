use judge_core::model::judge_output;
use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;

#[gen_stub_pyclass_enum]
#[pyclass(eq, eq_int, module = "traopy_util.util.v0")]
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum JudgeStatus {
    AC,
    WA,
    TLE,
    MLE,
    OLE,
    RE,
    CE,
}

#[pyfunction]
#[gen_stub_pyfunction(module = "traopy_util.util.v0")]
#[pyo3(signature = (status, time_ms, memory_kib, score, continue_next, message=None))]
pub fn jsonify_displayable_output(
    status: JudgeStatus,
    time_ms: f64,
    memory_kib: f64,
    score: i64,
    continue_next: bool,
    message: Option<String>,
) -> PyResult<String> {
    let inner_status = match status {
        JudgeStatus::AC => judge_output::JudgeStatus::AC,
        JudgeStatus::WA => judge_output::JudgeStatus::WA,
        JudgeStatus::TLE => judge_output::JudgeStatus::TLE,
        JudgeStatus::MLE => judge_output::JudgeStatus::MLE,
        JudgeStatus::OLE => judge_output::JudgeStatus::OLE,
        JudgeStatus::RE => judge_output::JudgeStatus::RE,
        JudgeStatus::CE => judge_output::JudgeStatus::CE,
    };
    let continue_status = if continue_next {
        judge_output::ContinueStatus::Continue
    } else {
        judge_output::ContinueStatus::Stop
    };
    let result = judge_output::DisplayableExecutionResult {
        status: inner_status,
        time: time_ms,
        memory: memory_kib,
        score,
        message,
        continue_status,
    };
    let wrapped = judge_output::ExecutionResult::Displayable(result);
    let json = serde_json::to_string(&wrapped)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{:?}", e)))?;
    Ok(json)
}

#[pyfunction]
#[gen_stub_pyfunction(module = "traopy_util.util.v0")]
pub fn jsonify_hidden_output(continue_next: bool) -> PyResult<String> {
    let continue_status = if continue_next {
        judge_output::ContinueStatus::Continue
    } else {
        judge_output::ContinueStatus::Stop
    };
    let result = judge_output::HiddenExecutionResult { continue_status };
    let wrapped = judge_output::ExecutionResult::Hidden(result);
    let json = serde_json::to_string(&wrapped)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{:?}", e)))?;
    Ok(json)
}
