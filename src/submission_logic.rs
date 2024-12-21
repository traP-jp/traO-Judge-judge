mod cmd_input_parser;
mod heuristics;
mod models;
mod single_run;
pub mod logic;

use crate::container::Container as ContainerTrait;
use crate::models::{judge_recipe::SubmissionInput, judge_result::SubmissionOutput};
use anyhow::Result;
use std::time::Duration;

pub trait Logic<ContainerType: ContainerTrait> {
    async fn judge(
        &self,
        judge: SubmissionInput,
        connection_time_limit: Duration,
        execution_time_limit: Duration,
    ) -> Result<SubmissionOutput>;
}
