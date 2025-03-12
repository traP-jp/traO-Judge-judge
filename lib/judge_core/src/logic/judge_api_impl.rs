use crate::logic::*;
use crate::model::*;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug)]
pub struct JudgeApiImpl<
    PRClient: problem_registry::ProblemRegistryClient,
    RToken: Send + Sync + 'static,
    OToken: Clone + Send + Sync + 'static,
    JobApi: job::JobApi<RToken, OToken>,
> {
    problem_registry_client: PRClient,
    job_api: JobApi,
    _phantom: std::marker::PhantomData<(Arc<RToken>, OToken)>,
}

impl<
        PRClient: problem_registry::ProblemRegistryClient,
        RToken: Send + Sync + 'static,
        OToken: Clone + Send + Sync + 'static,
        JobApi: job::JobApi<RToken, OToken>,
    > JudgeApiImpl<PRClient, RToken, OToken, JobApi>
{
    pub fn new(problem_registry_client: PRClient, job_api: JobApi) -> Self {
        Self {
            problem_registry_client,
            job_api,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<
        PRClient: problem_registry::ProblemRegistryClient,
        RToken: Send + Sync + 'static,
        OToken: Clone + Send + Sync + 'static,
        JobApi: job::JobApi<RToken, OToken>,
    > Clone for JudgeApiImpl<PRClient, RToken, OToken, JobApi>
{
    fn clone(&self) -> Self {
        Self {
            problem_registry_client: self.problem_registry_client.clone(),
            job_api: self.job_api.clone(),
            _phantom: std::marker::PhantomData,
        }
    }
}

#[axum::async_trait]
impl<
        PRClient: problem_registry::ProblemRegistryClient,
        RToken: Send + Sync + 'static,
        OToken: Clone + Send + Sync + 'static,
        JobApi: job::JobApi<RToken, OToken>,
    > judge::JudgeApi for JudgeApiImpl<PRClient, RToken, OToken, JobApi>
{
    async fn judge(&self, judge_request: judge::JudgeRequest) -> judge::JudgeResponse {
        let (runtime_procedure, identifier_map) = registered_procedure_converter::convert(
            &judge_request.procedure,
            &judge_request.runtime_texts,
        )?;
        let runner = runner::Runner::new(self.job_api.clone(), runtime_procedure).await?;
        let judge_results = runner.run().await?;
        let mut judge_results_depid = HashMap::new();
        for (runtime_id, result) in judge_results {
            let dep_id = identifier_map
                .get(&runtime_id)
                .ok_or(anyhow::anyhow!("DepId not found"))?
                .clone();
            judge_results_depid.insert(dep_id, result);
        }
        Ok(judge_results_depid)
    }
}
