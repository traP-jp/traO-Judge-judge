use super::tokens::{OutcomeToken, RegistrationToken};
use judge_core::model::{job, job::*, problem_registry};
use std::collections::HashMap;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::process::Output;
use uuid::Uuid;
use judge_core::constant::env_var_exec;

#[derive(Debug, Clone)]
pub struct JobApi<ProblemRegistryClient: problem_registry::ProblemRegistryClient> {
    temp_dir: PathBuf,
    problem_registry_client: ProblemRegistryClient,
}

impl<ProblemRegistryClient: problem_registry::ProblemRegistryClient> JobApi<ProblemRegistryClient> {
    pub fn new(
        temp_dir: PathBuf,
        problem_registry_client: ProblemRegistryClient,
    ) -> anyhow::Result<Self> {
        std::fs::create_dir_all(&temp_dir).map_err(|e| anyhow::anyhow!(e.to_string()))?;
        Ok(Self {
            temp_dir,
            problem_registry_client,
        })
    }
}

#[axum::async_trait]
impl<ProblemRegistryClient: problem_registry::ProblemRegistryClient>
    job::JobApi<RegistrationToken, OutcomeToken> for JobApi<ProblemRegistryClient>
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
            .map(|dep| (dep.envvar.clone(), dep.outcome.path().clone()))
            .collect::<HashMap<_, _>>();
        envvars.insert(env_var_exec::OUTPUT_PATH.to_string(), this.path().clone());
        let script_path = envvars
            .get(env_var_exec::SCRIPT_PATH)
            .ok_or(ExecutionError::InternalError(
                "No SCRIPT envvar".to_string(),
            ))?;
        std::fs::set_permissions(
            script_path,
            std::fs::Permissions::from_mode(0o755),
        )
            .map_err(|e| ExecutionError::InternalError(e.to_string()))?;
        let output = std::process::Command::new(script_path)
            .envs(&envvars)
            .output()
            .map_err(|e| ExecutionError::JudgeFailed(format!("{}: {}", e.to_string(), script_path.to_string_lossy())))?;
        Ok((this, output))
    }

    async fn place_file(&self, file_conf: FileConf) -> Result<OutcomeToken, FilePlacementError> {
        let id = Uuid::new_v4();
        let path = self.temp_dir.join(id.to_string());
        let outcome = OutcomeToken::new(path);
        match file_conf {
            FileConf::EmptyDirectory => {
                std::fs::create_dir(&outcome.path())
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
                std::fs::write(&outcome.path(), content)
                    .map_err(|e| FilePlacementError::PlaceFailed(e.to_string()))?;
            }
            FileConf::RuntimeText(content) => {
                std::fs::write(&outcome.path(), content)
                    .map_err(|e| FilePlacementError::PlaceFailed(e.to_string()))?;
            }
        }
        Ok(outcome)
    }
}
