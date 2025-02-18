use super::job_outcome::JobOutcome;
use futures::Future;
use judge_core::*;
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct JobApi<ProblemRegistryClient: problem_registry::ProblemRegistryClient + Clone> {
    temp_dir: PathBuf,
    problem_registry_client: ProblemRegistryClient,
}

impl<ProblemRegistryClient: problem_registry::ProblemRegistryClient + Clone>
    JobApi<ProblemRegistryClient>
{
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

    async fn run_future_internal(
        job_conf: job::ExecutionJob<JobOutcome>,
        src_dir: JobOutcome,
        script_file: JobOutcome,
    ) -> Result<job::ExecutionJobFinished<JobOutcome>, job::ExecutionJobError> {
        // set up environment variables
        let mut envvars = std::collections::HashMap::new();
        envvars.insert(
            "SRC".to_string(),
            src_dir.path().to_string_lossy().to_string(),
        );
        envvars.insert(
            "SCRIPT".to_string(),
            script_file.path().to_string_lossy().to_string(),
        );
        for mut dep in job_conf.depends_on_with_names {
            let dep_outcome = match dep.job_outcome_rx.recv().await {
                Ok(job::JobOutcomeAcquisitionResult::Succeeded(outcome)) => outcome,
                Ok(job::JobOutcomeAcquisitionResult::FailedExpectedly) => {
                    return Ok(job::ExecutionJobFinished::PrecedingJobFailedExpectedly);
                }
                Ok(job::JobOutcomeAcquisitionResult::FailedUnexpectedly(err_message)) => {
                    return Err(job::ExecutionJobError::InternalError(err_message));
                }
                Err(e) => {
                    return Err(job::ExecutionJobError::InternalError(e.to_string()));
                }
            };
            envvars.insert(
                dep.envvar_name,
                dep_outcome.path().to_string_lossy().to_string(),
            );
        }
        let output = std::process::Command::new(&script_file.path())
            .output()
            .map_err(|e| job::ExecutionJobError::InternalError(e.to_string()))?;
        Ok(job::ExecutionJobFinished::Succeeded(src_dir, output))
    }
}

impl<ProblemRegistryClient: problem_registry::ProblemRegistryClient + Clone> job::JobApi<JobOutcome>
    for JobApi<ProblemRegistryClient>
{
    async fn place_file(
        &self,
        file: job::FilePlacementJob,
    ) -> Result<JobOutcome, job::FilePlacementJobError> {
        let path = self.temp_dir.join(Uuid::new_v4().to_string());
        match file {
            job::FilePlacementJob::PlaceEmptyDirectory => {
                std::fs::create_dir(&path)
                    .map_err(|e| job::FilePlacementJobError::InternalError(e.to_string()))?;
            }
            job::FilePlacementJob::PlaceRuntimeTextFile(content) => {
                std::fs::write(&path, content)
                    .map_err(|e| job::FilePlacementJobError::InternalError(e.to_string()))?;
            }
            job::FilePlacementJob::PlaceTextFile(resource_id) => {
                let content = self
                    .problem_registry_client
                    .fetch(resource_id)
                    .await
                    .map_err(|e| match e {
                        problem_registry::ResourceFetchError::FetchFailed(err_message) => {
                            job::FilePlacementJobError::InternalError(err_message)
                        }
                        problem_registry::ResourceFetchError::NotFound(resource_id) => {
                            job::FilePlacementJobError::InvalidResourceId(resource_id)
                        }
                    })?;
                std::fs::write(&path, content)
                    .map_err(|e| job::FilePlacementJobError::InternalError(e.to_string()))?;
            }
        }
        Ok(JobOutcome::new(path))
    }

    async fn run_future(
        &self,
        job_conf: job::ExecutionJob<JobOutcome>,
        _: i32,
    ) -> Result<
        impl Future<Output = Result<job::ExecutionJobFinished<JobOutcome>, job::ExecutionJobError>>,
        job::ExecutionJobPreparationError,
    > {
        // prepare files
        let src_outcome = self
            .place_file(job::FilePlacementJob::PlaceEmptyDirectory)
            .await
            .map_err(|e| job::ExecutionJobPreparationError::InternalError(e.to_string()))?;
        let script_outcome = self
            .place_file(job::FilePlacementJob::PlaceRuntimeTextFile(
                job_conf.script.clone(),
            ))
            .await
            .map_err(|e| job::ExecutionJobPreparationError::InternalError(e.to_string()))?;

        let future = Self::run_future_internal(job_conf, src_outcome, script_outcome);
        Ok(future)
    }
}
