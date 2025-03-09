use std::net::Ipv4Addr;

use judge_core::job;
use uuid::Uuid;

trait AwsClient {
    async fn create_instance(&mut self, instance_id: Uuid) -> Result<Ipv4Addr, anyhow::Error>;
    async fn terminate_instance(&mut self, instance_id: Uuid) -> Result<(), anyhow::Error>;
    async fn place_file(
        &self,
        outcome_id: Uuid,
        file_conf: job::FileConf,
    ) -> Result<Uuid, anyhow::Error>;
    async fn push_outcome_to_instance_directory(
        &self,
        instance_id: Uuid,
        outcome_id: Uuid,
    ) -> Result<(), anyhow::Error>;
    async fn pull_outcome_from_instance_directory(
        &self,
        instance_id: Uuid,
        outcome_id: Uuid,
    ) -> Result<(), anyhow::Error>;
    async fn clear_instance_directory(&self, instance_id: Uuid) -> Result<(), anyhow::Error>;
    async fn remove_outcome_directory(&self, outcome_id: Uuid) -> Result<(), anyhow::Error>;
}
