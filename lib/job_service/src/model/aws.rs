use std::net::Ipv4Addr;

pub struct AwsInstanceInfo {
    pub aws_id: String,
    pub ip_addr: Ipv4Addr,
}

#[axum::async_trait]
pub trait AwsClient {
    async fn create_instance(&self) -> Result<AwsInstanceInfo, anyhow::Error>;
    async fn terminate_instance(&self, aws_id: String) -> Result<(), anyhow::Error>;
}
