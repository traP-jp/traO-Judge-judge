use anyhow::{ensure, Context};
use async_trait::async_trait;
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_ec2::{
    types::{IamInstanceProfileSpecification, InstanceType, Placement},
    Client as Ec2Client,
};
use aws_sdk_s3::Client as S3Client;
use base64::{prelude::BASE64_STANDARD, Engine};
use judge_core::model::job::FileConf;
use std::{collections::HashMap, env, fs::File, io::Write, net::Ipv4Addr, str::FromStr};
use uuid::Uuid;

#[async_trait]
pub trait AwsClient {
    async fn create_instance(&mut self, instance_id: Uuid) -> Result<Ipv4Addr, anyhow::Error>;
    async fn terminate_instance(&mut self, instance_id: Uuid) -> Result<(), anyhow::Error>;
    async fn place_file(
        &self,
        outcome_id: Uuid,
        file_name: Uuid,
        file_conf: FileConf,
    ) -> Result<(), anyhow::Error>;
}

struct AwsInstanceInfo {
    aws_id: String,
    ip_addr: Ipv4Addr,
}

pub struct AwsClientType {
    ec2_client: Ec2Client,
    aws_instance_table: HashMap<Uuid, AwsInstanceInfo>,
    s3_client: S3Client,
}

impl AwsClientType {
    pub async fn new() -> Self {
        // check env
        for key in [
            "AWS_ACCESS_KEY_ID",
            "AWS_SECRET_ACCESS_KEY",
            "SECURITY_GROUP_ID",
            "SUBNET_ID",
            "JUDGE_BUCKET_NAME",
            "EXEC_CONTAINER_IAM_ROLE",
        ] {
            if env::var(key).is_err() {
                panic!("{} is not set", key);
            }
        }

        let region_provider = RegionProviderChain::default_provider().or_else("us-west-2");
        let config = aws_config::from_env().region(region_provider).load().await;
        Self {
            ec2_client: Ec2Client::new(&config),
            aws_instance_table: HashMap::new(),
            s3_client: S3Client::new(&config),
        }
    }
}

#[async_trait]
impl AwsClient for AwsClientType {
    async fn create_instance(&mut self, instance_id: Uuid) -> Result<Ipv4Addr, anyhow::Error> {
        ensure!(
            !self.aws_instance_table.contains_key(&instance_id),
            "Instance already exists"
        );

        let security_group_id = env::var("SECURITY_GROUP_ID")?;
        let subnet_id = env::var("SUBNET_ID")?;

        let created_instance = self
            .ec2_client
            .run_instances()
            .image_id(
                "resolve:ssm:/aws/service/ami-amazon-linux-latest/al2023-ami-kernel-default-x86_64",
            )
            .instance_type(InstanceType::C6iLarge)
            .set_security_group_ids(Some(vec![security_group_id]))
            .set_subnet_id(Some(subnet_id))
            .user_data(
                BASE64_STANDARD
                    .encode(include_bytes!("../assets/user_data.sh"))
                    .to_string(),
            )
            .min_count(1)
            .max_count(1)
            .set_placement(Some(
                Placement::builder().availability_zone("us-west-2a").build(),
            ))
            .set_iam_instance_profile(Some(
                IamInstanceProfileSpecification::builder()
                    .arn(env::var("EXEC_CONTAINER_IAM_ROLE").unwrap().as_str())
                    .build(),
            ))
            .send()
            .await
            .context("Failed to create instance")?;
        ensure!(
            !created_instance.instances().is_empty(),
            "Failed to create instance"
        );

        let aws_id = created_instance.instances()[0]
            .instance_id()
            .context("Failed to get instance ID")?;
        let ip_addr_str = created_instance.instances()[0]
            .private_ip_address()
            .context("Failed to get private ip address")?;
        let ip_addr = Ipv4Addr::from_str(ip_addr_str).context("Failed to parse IP address")?;

        self.aws_instance_table.insert(
            instance_id,
            AwsInstanceInfo {
                aws_id: aws_id.to_string(),
                ip_addr,
            },
        );

        tracing::info!("Instance created: {aws_id}, {ip_addr}");

        Ok(ip_addr)
    }

    async fn terminate_instance(&mut self, instance_id: Uuid) -> Result<(), anyhow::Error> {
        let aws_id = self
            .aws_instance_table
            .get(&instance_id)
            .context("Failed to get instance ID from instance ID")?
            .aws_id
            .clone();
        let response = self
            .ec2_client
            .terminate_instances()
            .instance_ids(&aws_id)
            .send()
            .await
            .context("Failed to terminate instance")?;

        ensure!(
            !response.terminating_instances().is_empty(),
            "Failed to terminate instance"
        );

        self.aws_instance_table.remove(&instance_id);

        tracing::info!("Instance terminated: {aws_id}");

        Ok(())
    }
    async fn place_file(
        &self,
        outcome_id: Uuid,
        file_name: Uuid,
        file_conf: FileConf,
    ) -> Result<(), anyhow::Error> {
        match file_conf {
            FileConf::Text(resource_id) => {
                let result = self
                    .s3_client
                    .get_object()
                    .bucket("traO-judge") // TODO S3バケット名
                    .key(outcome_id.to_string() + "/" + file_name.to_string().as_str()) // TODO S3上のパス
                    .send()
                    .await?;
                let mut file = File::open(file_name.to_string())?; // TODO place先パス
                file.write_all(result.body.bytes().unwrap())?;
                Ok(())
            }
            FileConf::EmptyDirectory => {
                std::fs::create_dir_all(file_name.to_string())?; // TODO place先パス
                Ok(())
            }
            FileConf::RuntimeText(_) => {
                let result = self
                    .s3_client
                    .get_object()
                    .bucket("traO-judge") // TODO S3バケット名
                    .key(outcome_id.to_string() + "/" + file_name.to_string().as_str()) // TODO S3上のパス
                    .send()
                    .await?;
                let mut file = File::open(file_name.to_string())?; // TODO place先パス
                file.write_all(result.body.bytes().unwrap())?;
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;

    #[ignore]
    #[tokio::test]
    async fn test_create_instance() -> Result<(), anyhow::Error> {
        // lib/jobapi/.env を読み込む
        dotenv().ok();
        let mut client = AwsClientType::new().await;
        client.create_instance(Uuid::now_v7()).await?;
        Ok(())
    }

    #[ignore]
    #[tokio::test]
    async fn test_terminate_instance() -> Result<(), anyhow::Error> {
        // lib/jobapi/.env を読み込む
        dotenv().ok();
        let mut client = AwsClientType::new().await;
        let instance_id = Uuid::now_v7();
        client.aws_instance_table.insert(
            instance_id,
            AwsInstanceInfo {
                aws_id: "i-*****************".to_string(),
                ip_addr: Ipv4Addr::from_str("***.***.***.***")?,
            },
        );
        client.terminate_instance(instance_id).await?;
        Ok(())
    }
}
