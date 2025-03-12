use crate::model::{procedure::*, *};

use std::collections::HashMap;
use anyhow::Result;

#[axum::async_trait]
pub trait JudgeApi: Clone + Send + Sync + 'static {
    async fn judge(&self, request: JudgeRequest) -> Result<JudgeResponse>;
}

#[derive(Debug, Clone)]
pub struct JudgeRequest {
    pub procedure: registered::Procedure,
    pub runtime_texts: HashMap<String, String>,
}

pub type JudgeResponse =
    anyhow::Result<HashMap<identifiers::DepId, judge_output::ExecutionJobResult>>;
