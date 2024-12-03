use anyhow::Result;

pub struct SuccessfulExecutionOutput {
    stdout: String,
    stderr: String,
    exit_code: i32,
}

pub enum ExecutionOutput {
    Success(SuccessfulExecutionOutput),
    FailureByWriter(anyhow::Error),
}

pub trait RemoteExecutor {
    fn execute(
        &self,
        cmd: &str,
        envs: std::collections::HashMap<String, String>,
        connection_time_limit: std::time::Duration,
        execution_time_limit: std::time::Duration
    ) -> Result<ExecutionOutput>;
}