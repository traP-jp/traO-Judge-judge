pub mod logic;
mod job_scheduler;
mod cmd_input_parser;

use crate::container::Container as ContainerTrait;
use crate::models::{judge_recipe::SubmissionInput, judge_result::SubmissionOutput};
use anyhow::Result;
use uuid::Uuid;

pub trait Logic<ContainerType: ContainerTrait, JudgeOrderingType: Ord + Clone> {
    async fn exec(&self, judge: SubmissionInput<JudgeOrderingType>) -> Result<SubmissionOutput>;
    async fn add_container(&self, id: Uuid, container: ContainerType) -> Result<()>;
    async fn release_container(&self, id: Uuid) -> Result<()>;
}
