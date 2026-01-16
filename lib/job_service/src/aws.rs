use anyhow::{Context, ensure};
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_ec2::types::{BlockDeviceMapping, EbsBlockDevice, VolumeType};
use aws_sdk_ec2::{
    Client as Ec2Client,
    types::{IamInstanceProfileSpecification, InstanceType, Placement},
};
use base64::{Engine, prelude::BASE64_STANDARD};
use std::{env, net::Ipv4Addr, str::FromStr};

use crate::model::aws::{self, AwsInstanceInfo};

#[derive(Clone)]
pub struct AwsClient {
    // Cheap to clone since Ec2Client internally uses Arc.
    ec2_client: Ec2Client,
}

impl AwsClient {
    pub async fn new() -> Self {
        // check env
        for key in [
            "SECURITY_GROUP_ID",
            "SUBNET_ID",
            "JUDGE_BUCKET_NAME",
            "EXEC_CONTAINER_IAM_ROLE",
            "EXEC_INSTANCE_AMI",
            "DOCKER_IMAGE_NAME",
        ] {
            if env::var(key).is_err() {
                panic!("{} is not set", key);
            }
        }

        let region_provider = RegionProviderChain::default_provider().or_else("us-west-2");
        let config = aws_config::from_env().region(region_provider).load().await;
        Self {
            ec2_client: Ec2Client::new(&config),
        }
    }
}

#[axum::async_trait]
impl aws::AwsClient for AwsClient {
    async fn create_instance(&self) -> Result<AwsInstanceInfo, anyhow::Error> {
        let security_group_id = env::var("SECURITY_GROUP_ID")?;
        let subnet_id = env::var("SUBNET_ID")?;
        let ami_id = env::var("EXEC_INSTANCE_AMI")?;
        let docker_image_name = env::var("DOCKER_IMAGE_NAME")?;

        let created_instance = self
            .ec2_client
            .run_instances()
            .image_id(ami_id)
            .instance_type(InstanceType::C6iLarge)
            .set_security_group_ids(Some(vec![security_group_id]))
            .set_subnet_id(Some(subnet_id))
            .user_data(
                BASE64_STANDARD
                    .encode(format!(
                        "#!/bin/bash
                        sudo aws s3 cp s3://trao-infra-resources/exec-app/exec-app /root/exec-app >> /log.txt 2>&1
                        sudo chmod +x /root/exec-app
                        DOCKER_IMAGE_NAME={} RUST_LOG=TRACE /root/exec-app >> /log.txt 2>&1",
                        docker_image_name
                    ))
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
            .block_device_mappings(
                BlockDeviceMapping::builder()
                    .device_name("/dev/xvda")
                    .ebs(
                        EbsBlockDevice::builder()
                            .volume_size(32)
                            .delete_on_termination(true)
                            .volume_type(VolumeType::Gp3)
                            .build(),
                    )
                    .build(),
            )
            .send()
            .await
            .context("Failed to create instance")?;
        ensure!(
            !created_instance.instances().is_empty(),
            "Failed to create instance"
        );

        let aws_id = created_instance.instances()[0]
            .instance_id()
            .context("Failed to get instance ID")?
            .to_string();
        let ip_addr_str = created_instance.instances()[0]
            .private_ip_address()
            .context("Failed to get private ip address")?;
        let ip_addr = Ipv4Addr::from_str(ip_addr_str).context("Failed to parse IP address")?;

        tracing::info!("Instance created: {aws_id}, {ip_addr}");

        Ok(AwsInstanceInfo { aws_id, ip_addr })
    }

    async fn terminate_instance(&self, aws_id: String) -> Result<(), anyhow::Error> {
        let response = self
            .ec2_client
            .terminate_instances()
            .instance_ids(&aws_id)
            .send()
            .await
            .context("Failed to terminate instance")?;

        ensure!(
            !response.terminating_instances().is_empty(),
            "Failed to terminate instance: no value was sent"
        );

        tracing::info!("Instance terminated: {aws_id}");

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::model::aws::AwsClient as _;

    use super::*;
    use dotenv::dotenv;

    #[ignore]
    #[tokio::test]
    async fn test_create_instance() -> Result<(), anyhow::Error> {
        // lib/job_service/.env を読み込む
        dotenv().ok();
        let client = AwsClient::new().await;
        client.create_instance().await?;
        Ok(())
    }

    #[ignore]
    #[tokio::test]
    async fn test_terminate_instance() -> Result<(), anyhow::Error> {
        // lib/job_service/.env を読み込む
        dotenv().ok();
        let client = AwsClient::new().await;
        client
            .terminate_instance("i-*****************".to_string())
            .await?;
        Ok(())
    }
}
