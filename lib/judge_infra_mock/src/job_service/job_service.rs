use super::tokens::{OutcomeToken, RegistrationToken};
use bollard::container::StartContainerOptions;
use bollard::{
    Docker,
    container::{Config, CreateContainerOptions, WaitContainerOptions},
    service::{HostConfig, Mount},
};
use futures::StreamExt;
use judge_core::constant::env_var_exec;
use judge_core::model::{job, job::*, problem_registry};
use std::collections::HashMap;
use std::os::unix::process::ExitStatusExt;
use std::path::PathBuf;
use std::process::Output;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct JobService<ProblemRegistryClient: problem_registry::ProblemRegistryClient> {
    host_temp_dir: PathBuf,
    container_temp_dir: PathBuf,
    problem_registry_client: ProblemRegistryClient,
    container_image_name: String,
}

impl<ProblemRegistryClient: problem_registry::ProblemRegistryClient> JobService<ProblemRegistryClient> {
    pub fn new(
        host_temp_dir: PathBuf,
        container_temp_dir: PathBuf,
        problem_registry_client: ProblemRegistryClient,
        container_image_name: String,
    ) -> anyhow::Result<Self> {
        std::fs::create_dir_all(&host_temp_dir).map_err(|e| anyhow::anyhow!(e.to_string()))?;
        Ok(Self {
            host_temp_dir,
            container_temp_dir,
            problem_registry_client,
            container_image_name,
        })
    }
}

#[axum::async_trait]
impl<ProblemRegistryClient: problem_registry::ProblemRegistryClient>
    job::JobService<RegistrationToken, OutcomeToken> for JobService<ProblemRegistryClient>
{
    async fn reserve_execution(
        &self,
        count: usize,
    ) -> Result<Vec<RegistrationToken>, ReservationError> {
        Ok((0..count)
            .map(|_| RegistrationToken { _marker: () })
            .collect())
    }

    async fn execute(
        &self,
        _: RegistrationToken,
        dependencies: Vec<Dependency<OutcomeToken>>,
    ) -> Result<(OutcomeToken, Output), ExecutionError> {
        let this = self
            .place_file(FileConf::EmptyDirectory)
            .await
            .map_err(|e| ExecutionError::InternalError(e.to_string()))?;
        let mut envvars = dependencies
            .iter()
            .map(|dep| {
                (
                    dep.envvar.clone(),
                    self.container_temp_dir.join(dep.outcome.path()),
                )
            })
            .collect::<HashMap<_, _>>();
        envvars.insert(
            env_var_exec::OUTPUT_PATH.to_string(),
            self.container_temp_dir.join(this.path()),
        );
        let script_path =
            envvars
                .get(env_var_exec::SCRIPT_PATH)
                .ok_or(ExecutionError::InternalError(
                    "No SCRIPT envvar".to_string(),
                ))?;
        let stdout_rel_path = PathBuf::from(uuid::Uuid::new_v4().to_string());
        let stderr_rel_path = PathBuf::from(uuid::Uuid::new_v4().to_string());
        let stdout_container_path = self.container_temp_dir.join(stdout_rel_path.clone());
        let stderr_container_path = self.container_temp_dir.join(stderr_rel_path.clone());
        let stdout_host_path = self.host_temp_dir.join(stdout_rel_path.clone());
        let stderr_host_path = self.host_temp_dir.join(stderr_rel_path.clone());
        let docker = Docker::connect_with_local_defaults()
            .map_err(|e| ExecutionError::InternalError(e.to_string()))?;
        let cmd_inner = format!(
            "chmod +x {} && {} > {} 2> {}",
            script_path.display(),
            script_path.display(),
            stdout_container_path.display(),
            stderr_container_path.display()
        );
        let envvars_vec = envvars
            .iter()
            .map(|(k, v)| format!("{}={}", k, v.display()))
            .collect::<Vec<_>>();
        let config = Config {
            image: Some(self.container_image_name.as_str()),
            env: Some(envvars_vec.iter().map(|x| x.as_str()).collect()),
            cmd: Some(vec!["/bin/sh", "-c", cmd_inner.as_str()]),
            host_config: Some(HostConfig {
                binds: Some(vec![format!(
                    "{}:{}",
                    self.host_temp_dir.display(),
                    self.container_temp_dir.display()
                )]),
                ..Default::default()
            }),
            ..Default::default()
        };
        let container_name: String = uuid::Uuid::new_v4().to_string();
        let container = docker
            .create_container(
                Some(CreateContainerOptions {
                    name: container_name.as_str(),
                    platform: Some("linux/amd64"),
                }),
                config,
            )
            .await
            .map_err(|e| ExecutionError::InternalError(e.to_string()))?;
        docker
            .start_container(&container.id, None::<StartContainerOptions<String>>)
            .await
            .map_err(|e| ExecutionError::InternalError(e.to_string()))?;
        let wait = docker
            .wait_container(&container_name, None::<WaitContainerOptions<String>>)
            .next()
            .await
            .ok_or(ExecutionError::InternalError(
                "Failed to wait for container".to_string(),
            ))?
            .map_err(|e| ExecutionError::InternalError(e.to_string()))?;
        let exit_code = wait.status_code;
        let stdout_string = std::fs::read_to_string(&stdout_host_path)
            .map_err(|e| ExecutionError::InternalError(e.to_string()))?;
        let stderr_string = std::fs::read_to_string(&stderr_host_path)
            .map_err(|e| ExecutionError::InternalError(e.to_string()))?;
        let stdout = stdout_string.into_bytes();
        let stderr = stderr_string.into_bytes();
        let output = Output {
            status: std::process::ExitStatus::from_raw(exit_code as i32),
            //            status: std::process::ExitStatus::from_raw(0),
            stdout,
            stderr,
        };
        Ok((this, output))
    }

    async fn place_file(&self, file_conf: FileConf) -> Result<OutcomeToken, FilePlacementError> {
        let id = Uuid::new_v4();
        let rel_path = PathBuf::from(id.to_string());
        let path = self.host_temp_dir.join(rel_path.clone());
        let outcome = OutcomeToken::new(rel_path);
        match file_conf {
            FileConf::EmptyDirectory => {
                std::fs::create_dir(&path)
                    .map_err(|e| FilePlacementError::PlaceFailed(e.to_string()))?;
            }
            FileConf::Text(resource_id) => {
                let content = self
                    .problem_registry_client
                    .fetch(resource_id)
                    .await
                    .map_err(|e| match e {
                        problem_registry::ResourceFetchError::NotFound(id) => {
                            FilePlacementError::InvalidResourceId(id)
                        }
                        _ => FilePlacementError::PlaceFailed(e.to_string()),
                    })?;
                std::fs::write(&path, content)
                    .map_err(|e| FilePlacementError::PlaceFailed(e.to_string()))?;
            }
            FileConf::RuntimeText(content) => {
                std::fs::write(&path, content)
                    .map_err(|e| FilePlacementError::PlaceFailed(e.to_string()))?;
            }
        }
        Ok(outcome)
    }
}
