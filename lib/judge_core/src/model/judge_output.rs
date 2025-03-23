use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
pub enum JudgeStatus {
    AC,
    WA,
    TLE,
    MLE,
    OLE,
    RE,
    CE,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ContinueStatus {
    Continue,
    Stop,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayableExecutionResult {
    pub status: JudgeStatus,
    pub time: f64,
    pub memory: f64,
    pub score: i64,
    pub message: Option<String>,
    pub continue_status: ContinueStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HiddenExecutionResult {
    pub continue_status: ContinueStatus,
}

/// ExecutionResult will be returned from exec container as stdout
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExecutionResult {
    /// Frontend-displayable execution result
    Displayable(DisplayableExecutionResult),
    /// Not displayed to frontend (e.g. for validation)
    Hidden(HiddenExecutionResult),
}

/// ExecutionJobResult is the return value of judge-control
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExecutionJobResult {
    /// Execution result
    ExecutionResult(ExecutionResult),
    /// Early exit
    EarlyExit,
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum ExecutionOutputParseError {
    #[error("Invalid JSON: {0}")]
    InvalidJson(String),
    #[error("Non-zero exit code")]
    NonZeroExitCode,
}

pub fn parse(output: &std::process::Output) -> Result<ExecutionResult, ExecutionOutputParseError> {
    let stdout = String::from_utf8(output.stdout.clone())
        .map_err(|e| ExecutionOutputParseError::InvalidJson(e.to_string()))?;
    if !output.status.success() {
        return Err(ExecutionOutputParseError::NonZeroExitCode);
    }
    let execution_result: ExecutionResult = serde_json::from_str(&stdout)
        .map_err(|e| ExecutionOutputParseError::InvalidJson(e.to_string()))?;
    Ok(execution_result)
}
