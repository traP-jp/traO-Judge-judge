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
    WE,
}

#[gen_stub_pyclass]
#[pyclass(module = "traopy_util.util.v0")]
#[derive(Debug, Clone)]
pub struct ExecutionResult {
    pub status: JudgeStatus,
    pub time: f64,
    pub memory: f64,
    pub score: i64,
}

#[gen_stub_pymethods]
#[pymethods]
impl ExecutionResult {
    #[new]
    fn new(
        status: JudgeStatus,
        time: f64,
        memory: f64,
        score: i64,
    ) -> Self {
        ExecutionResult {
            status,
            time,
            memory,
            score,
        }
    }
    #[getter]
    fn status(&self) -> JudgeStatus {
        self.status.clone()
    }
    #[getter]
    fn time(&self) -> f64 {
        self.time
    }
    #[getter]
    fn memory(&self) -> f64 {
        self.memory
    }
    #[getter]
    fn score(&self) -> i64 {
        self.score
    }
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
        JudgeStatus::WE => judge_output::JudgeStatus::WE,
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

#[pyfunction]
#[gen_stub_pyfunction(module = "traopy_util.util.v0")]
pub fn dejsonify_output(
    json: String,
) -> PyResult<Option<ExecutionResult>> {
    let inner_output = serde_json::from_str::<judge_output::ExecutionResult>(&json)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{:?}", e)))?;
    let result = match inner_output {
        judge_output::ExecutionResult::Displayable(result) => {
            let status = match result.status {
                judge_output::JudgeStatus::AC => JudgeStatus::AC,
                judge_output::JudgeStatus::WA => JudgeStatus::WA,
                judge_output::JudgeStatus::TLE => JudgeStatus::TLE,
                judge_output::JudgeStatus::MLE => JudgeStatus::MLE,
                judge_output::JudgeStatus::OLE => JudgeStatus::OLE,
                judge_output::JudgeStatus::RE => JudgeStatus::RE,
                judge_output::JudgeStatus::CE => JudgeStatus::CE,
                judge_output::JudgeStatus::WE => JudgeStatus::WE,
            };
            Some(ExecutionResult {
                status,
                time: result.time,
                memory: result.memory,
                score: result.score,
            })
        }
        _ => None,
    };
    Ok(result)
}

#[pyfunction]
#[gen_stub_pyfunction(module = "traopy_util.util.v0")]
// WE > CE > RE > OLE > WA > TLE > MLE > AC
pub fn merge_judge_status(statuses: Vec<JudgeStatus>) -> PyResult<JudgeStatus> {
    let mut status_idx = 7;
    for status in statuses {
        let idx = match status {
            JudgeStatus::WE => 0,
            JudgeStatus::CE => 1,
            JudgeStatus::RE => 2,
            JudgeStatus::OLE => 3,
            JudgeStatus::WA => 4,
            JudgeStatus::TLE => 5,
            JudgeStatus::MLE => 6,
            JudgeStatus::AC => 7,
        };
        if idx < status_idx {
            status_idx = idx;
        }
    }
    let status = match status_idx {
        0 => JudgeStatus::WE,
        1 => JudgeStatus::CE,
        2 => JudgeStatus::RE,
        3 => JudgeStatus::OLE,
        4 => JudgeStatus::WA,
        5 => JudgeStatus::TLE,
        6 => JudgeStatus::MLE,
        7 => JudgeStatus::AC,
        _ => unreachable!(),
    };
    Ok(status)
}
