use std::net::Ipv4Addr;

use uuid::Uuid;

#[axum::async_trait]
pub trait AwsClient {
    async fn create_instance(&mut self, instance_id: Uuid) -> Result<Ipv4Addr, anyhow::Error>;
    async fn terminate_instance(&mut self, instance_id: Uuid) -> Result<(), anyhow::Error>;
}
