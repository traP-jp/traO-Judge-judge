use std::{net::Ipv4Addr, path::PathBuf};

use uuid::Uuid;

trait AwsClient: Sized {
    async fn new() -> Result<Self, anyhow::Error>;
    async fn create_instance(&mut self, instance_id: Uuid) -> Result<Ipv4Addr, anyhow::Error>;
    async fn terminate_instance(&mut self, instance_id: Uuid) -> Result<(), anyhow::Error>;
    async fn touch_file(&self, instance_id: Uuid, file_path: PathBuf) -> Result<(), anyhow::Error>;
    async fn remove_file(&self, instance_id: Uuid, file_path: PathBuf)
        -> Result<(), anyhow::Error>;
}
