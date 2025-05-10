use crate::model::judge_output::*;

pub fn parse(output: &std::process::Output) -> Result<ExecutionResult, ExecutionOutputParseError> {
    let stdout = String::from_utf8(output.stdout.clone())
        .map_err(|e| ExecutionOutputParseError::InvalidJson(e.to_string()))?;
    if !output.status.success() {
        return Err(ExecutionOutputParseError::NonZeroExitCode(format!(
            "stdout: {}, stderr: {}, exit code: {}",
            stdout,
            String::from_utf8(output.stderr.clone()).unwrap_or_default(),
            output.status.code().unwrap_or(-1)
        )));
    }
    let execution_result: ExecutionResult = serde_json::from_str(&stdout)
        .map_err(|e| ExecutionOutputParseError::InvalidJson(e.to_string()))?;
    Ok(execution_result)
}
