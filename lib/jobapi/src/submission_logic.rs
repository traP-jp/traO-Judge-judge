mod extra_envs;
mod file_preparation;
mod heuristics;
pub mod logic;
mod models;
mod output_parser;
mod single_run;

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
