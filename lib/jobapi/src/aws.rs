use anyhow::anyhow;
use async_trait::async_trait;
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_ec2::operation::describe_instances::DescribeInstancesOutput;
use aws_sdk_ec2::types::{InstanceType, Placement};
use aws_sdk_ec2::Client as Ec2Client;
use aws_sdk_s3::Client as S3Client;
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use judge_core::job;
use judge_core::job::FileConf;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Write;
use std::net::Ipv4Addr;
use std::str::FromStr;
use uuid::Uuid;

#[async_trait]
pub trait AwsClient {
    async fn create_instance(&mut self, instance_id: Uuid) -> Result<Ipv4Addr, anyhow::Error>;
    async fn initialize_instance(&mut self, instance_id: Uuid) -> Result<(), anyhow::Error>;
    async fn terminate_instance(&mut self, instance_id: Uuid) -> Result<(), anyhow::Error>;
    async fn place_file(
        &self,
        outcome_id: Uuid,
        file_name: Uuid,
        file_conf: job::FileConf,
    ) -> Result<(), anyhow::Error>;
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

struct AwsInstanceInfo {
    aws_id: String,
    ip_addr: Option<Ipv4Addr>,
    initialized: bool,
}

pub struct AwsClientType {
    ec2_client: Ec2Client,
    aws_instance_table: HashMap<Uuid, AwsInstanceInfo>,
    max_instance_count: usize,
    s3_client: S3Client,
}

impl AwsClientType {
    pub async fn new() -> Self {
        // check env
        for key in [
            "AWS_ACCESS_KEY_ID",
            "AWS_SECRET_ACCESS_KEY",
            "SECURITY_GROUP_ID",
            "VOLUME_ID",
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
            max_instance_count: 15,
            s3_client: S3Client::new(&config),
        }
    }
}

#[async_trait]
impl AwsClient for AwsClientType {
    async fn create_instance(&mut self, instance_id: Uuid) -> Result<Ipv4Addr, anyhow::Error> {
        if self.aws_instance_table.len() >= self.max_instance_count {
            return Err(anyhow!("Too many instances"));
        }
        if self.aws_instance_table.contains_key(&instance_id) {
            return Err(anyhow!("Instance already exists"));
        }

        let security_group_id =
            env::var("SECURITY_GROUP_ID").expect("SECURITY_GROUP_ID is not set");

        let read_file_base64 = |file_path: &str| {
            let file = std::fs::read(file_path).expect("Failed to read file");
            Ok::<String, anyhow::Error>(BASE64_STANDARD.encode(file).to_string())
        };

        let created_instance = self
            .ec2_client
            .run_instances()
            .image_id(
                "resolve:ssm:/aws/service/ami-amazon-linux-latest/al2023-ami-kernel-default-x86_64",
            )
            .instance_type(InstanceType::C6iLarge)
            .set_security_group_ids(Some(vec![security_group_id]))
            .user_data(read_file_base64("assets/user_data.sh").expect("Failed to read user data"))
            .min_count(1)
            .max_count(1)
            .set_placement(Some(
                Placement::builder().availability_zone("us-west-2a").build(),
            ))
            .send()
            .await
            .expect("Failed to create instance");
        if created_instance.instances().is_empty() {
            return Err(anyhow!("Failed to create instance"));
        }

        let aws_id = created_instance.instances()[0]
            .instance_id()
            .expect("Failed to get instance ID");

        println!("Created {aws_id}.");

        self.aws_instance_table.insert(
            instance_id,
            AwsInstanceInfo {
                aws_id: aws_id.to_string(),
                ip_addr: None,
                initialized: false,
            },
        );

        println!("Waiting for instance to be ready...");
        let ip_address = {
            let mut describe_instances: DescribeInstancesOutput;
            let mut public_ip: Option<&str>;
            loop {
                describe_instances = self
                    .ec2_client
                    .describe_instances()
                    .instance_ids(aws_id)
                    .send()
                    .await
                    .expect("Failed to describe instance");
                if describe_instances.reservations().is_empty() {
                    return Err(anyhow!("Failed to describe instance"));
                }

                public_ip = describe_instances.reservations()[0].instances()[0].public_ip_address();
                if public_ip.is_none() {
                    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                    continue;
                }
                break;
            }
            Ipv4Addr::from_str(public_ip.unwrap()).expect("Failed to parse IP address")
        };
        self.aws_instance_table
            .get_mut(&instance_id)
            .unwrap()
            .ip_addr = Some(ip_address);

        println!("Public IP: {}", ip_address);
        Ok(ip_address)
    }
    async fn initialize_instance(&mut self, instance_id: Uuid) -> Result<(), anyhow::Error> {
        let info = self.aws_instance_table.get(&instance_id).unwrap();
        if info.initialized {
            return Err(anyhow!("Instance is already initialized"));
        }
        let aws_id = &info.aws_id;
        let ip_address = &info.ip_addr.expect("No ip address");

        println!("Attaching volume...");

        // ボリュームをアタッチ

        let volume_id = {
            let volume_id = env::var("VOLUME_ID");
            if volume_id.is_err() {
                self.terminate_instance(instance_id).await?;
                return Err(anyhow!("VOLUME_ID is not set"));
            }
            volume_id?
        };

        {
            let response = self
                .ec2_client
                .attach_volume()
                .device("/dev/sdb")
                .instance_id(aws_id)
                .volume_id(volume_id)
                .send()
                .await;
            if response.is_err() {
                self.terminate_instance(instance_id).await?;
                return Err(anyhow!(
                    "Failed to attach volume: {}",
                    response.err().unwrap()
                ));
            }
        }
        let http_client = reqwest::Client::new();
        http_client
            .post(&format!("http://{}/mount", ip_address))
            .body(instance_id.to_string()) // TODO インスタンス用フォルダの名前を伝える
            .send()
            .await?;
        self.aws_instance_table
            .get_mut(&instance_id)
            .unwrap()
            .initialized = true;
        Ok(())
    }
    async fn terminate_instance(&mut self, instance_id: Uuid) -> Result<(), anyhow::Error> {
        let response = self
            .ec2_client
            .terminate_instances()
            .instance_ids(
                self.aws_instance_table
                    .get(&instance_id)
                    .expect("Failed to get instance ID from instance ID")
                    .aws_id
                    .clone(),
            )
            .send()
            .await
            .expect("Failed to terminate instance");
        if response.terminating_instances().is_empty() {
            return Err(anyhow!("Failed to terminate instance"));
        }
        self.aws_instance_table.remove(&instance_id);
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
            _ => todo!(),
        }
    }
    async fn push_outcome_to_instance_directory(
        &self,
        instance_id: Uuid,
        outcome_id: Uuid,
    ) -> Result<(), anyhow::Error> {
        todo!()
    }
    async fn pull_outcome_from_instance_directory(
        &self,
        instance_id: Uuid,
        outcome_id: Uuid,
    ) -> Result<(), anyhow::Error> {
        todo!()
    }
    async fn clear_instance_directory(&self, instance_id: Uuid) -> Result<(), anyhow::Error> {
        todo!()
    }
    async fn remove_outcome_directory(&self, outcome_id: Uuid) -> Result<(), anyhow::Error> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;

    #[tokio::test]
    async fn test_create_instance() -> Result<(), anyhow::Error> {
        // lib/jobapi/.env を読み込む
        dotenv().ok();
        let mut client = AwsClientType::new().await;
        client.create_instance(Uuid::now_v7()).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_initialize_instance() -> Result<(), anyhow::Error> {
        // lib/jobapi/.env を読み込む
        dotenv().ok();
        let mut client = AwsClientType::new().await;
        let instance_id = Uuid::now_v7();
        client.aws_instance_table.insert(
            instance_id,
            AwsInstanceInfo {
                aws_id: "i-*****************".to_string(),
                ip_addr: Some(Ipv4Addr::from_str("***.***.***.***")?),
                initialized: false,
            },
        );
        client.initialize_instance(instance_id).await?;
        Ok(())
    }

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
                ip_addr: Some(Ipv4Addr::from_str("***.***.***.***")?),
                initialized: false,
            },
        );
        client.terminate_instance(instance_id).await?;
        Ok(())
    }
}
