mod cmd_input_parser;
mod job_scheduler;
pub mod logic;

use crate::container::Container as ContainerTrait;
use crate::models::{judge_recipe::SubmissionInput, judge_result::SubmissionOutput};
use anyhow::Result;
use std::time::Duration;
use uuid::Uuid;

pub trait Logic<ContainerType: ContainerTrait, JudgeOrderingType: Ord + Clone> {
    async fn exec(
        &self,
        judge: SubmissionInput<JudgeOrderingType>,
        connection_time_limit: Duration,
        execution_time_limit: Duration,
    ) -> Result<SubmissionOutput>;
    async fn add_container(&self, id: Uuid, container: ContainerType) -> Result<()>;
    async fn release_container(&self, id: Uuid) -> Result<()>;
}
