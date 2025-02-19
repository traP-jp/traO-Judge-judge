use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
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
}

/// This returns from exec container as stdout
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExecutionResult {
    /// Frontend-displayable execution result
    Displayable(DisplayableExecutionResult),
    /// Not displayed to frontend (e.g. for validation)
    Hidden,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionReport {
    pub result: ExecutionResult,
    pub continue_status: ContinueStatus,
}

/// This is the final response from judge
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExecutionResponse {
    Report(ExecutionReport),
    EarlyExit,
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum ExecutionOutputParseError {
    #[error("Invalid JSON: {0}")]
    InvalidJson(String),
    #[error("Non-zero exit code")]
    NonZeroExitCode,
}

pub fn parse(output: &std::process::Output) -> Result<ExecutionReport, ExecutionOutputParseError> {
    let stdout = String::from_utf8(output.stdout.clone())
        .map_err(|e| ExecutionOutputParseError::InvalidJson(e.to_string()))?;
    if !output.status.success() {
        return Err(ExecutionOutputParseError::NonZeroExitCode);
    }
    let execution_report: ExecutionReport = serde_json::from_str(&stdout)
        .map_err(|e| ExecutionOutputParseError::InvalidJson(e.to_string()))?;
    Ok(execution_report)
}
