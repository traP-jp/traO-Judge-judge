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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContinueStatus {
    Continue,
    Stop,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayableJudgeResult {
    pub status: JudgeStatus,
    pub time: f64,
    pub memory: f64,
    pub score: i64,
    pub message: Option<String>,
    pub continue_status: ContinueStatus,
}

/// This returns from exec container as stdout
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum JudgeReport {
    /// Frontend-displayable execution result
    Displayable(DisplayableJudgeResult),
    /// Not displayed to frontend (e.g. for validation)
    Hidden,
}

/// This is the final response from judge
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExecutionResponse {
    Report(JudgeReport),
    EarlyExit,
    Error(String),
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum JudgeOutputParseError {
    #[error("Invalid JSON: {0}")]
    InvalidJson(String),
    #[error("Non-zero exit code")]
    NonZeroExitCode,
}

pub fn parse(output: &std::process::Output) -> Result<JudgeReport, JudgeOutputParseError> {
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    if !output.status.success() {
        return Err(JudgeOutputParseError::NonZeroExitCode);
    }
    let judge_report: JudgeReport = serde_json::from_str(&stdout)
        .map_err(|e| JudgeOutputParseError::InvalidJson(e.to_string()))?;
    Ok(judge_report)
}
