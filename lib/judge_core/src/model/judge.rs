use crate::model::{procedure::*, *};

use std::collections::HashMap;

#[axum::async_trait]
pub trait JudgeService: Clone + Send + Sync + 'static {
    async fn judge(&self, request: JudgeRequest) -> Result<JudgeResponse, JudgeError>;
}

#[derive(Debug, Clone)]
pub struct JudgeRequest {
    pub procedure: registered::Procedure,
    pub runtime_texts: HashMap<String, Vec<u8>>,
}

/// ExecutionOutput will be returned from exec container as stdout
#[derive(Debug, Clone)]
pub struct ExecutionOutput {
    pub stdout: Option<Vec<u8>>,
}

/// ExecutionOutputs
#[derive(Debug, Clone)]
pub struct ExecutionOutputs {
    pub outputs: HashMap<identifiers::DepId, ExecutionOutput>,
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum WriterProcessError {
    #[error("Writer process failed: {0}")]
    ProcessFailed(String),
}

#[derive(Debug, Clone)]
pub enum JudgeResponse {
    Success(ExecutionOutputs),
    WriterProcessError(WriterProcessError),
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum JudgeError {
    #[error("Internal error: {0}")]
    Internal(String),
}
