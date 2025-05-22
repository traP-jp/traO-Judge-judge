use crate::logic::*;
use crate::model::*;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug)]
pub struct JudgeServiceImpl<
    RToken: Send + Sync + 'static,
    OToken: Clone + Send + Sync + 'static,
    JobApi: job::JobApi<RToken, OToken>,
> {
    job_api: JobApi,
    _phantom: std::marker::PhantomData<(Arc<RToken>, OToken)>,
}

impl<
    RToken: Send + Sync + 'static,
    OToken: Clone + Send + Sync + 'static,
    JobApi: job::JobApi<RToken, OToken>,
> JudgeServiceImpl<RToken, OToken, JobApi>
{
    pub fn new(job_api: JobApi) -> Self {
        Self {
            job_api,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<
    RToken: Send + Sync + 'static,
    OToken: Clone + Send + Sync + 'static,
    JobApi: job::JobApi<RToken, OToken>,
> Clone for JudgeServiceImpl<RToken, OToken, JobApi>
{
    fn clone(&self) -> Self {
        Self {
            job_api: self.job_api.clone(),
            _phantom: std::marker::PhantomData,
        }
    }
}

#[axum::async_trait]
impl<
    RToken: Send + Sync + 'static,
    OToken: Clone + Send + Sync + 'static,
    JobApi: job::JobApi<RToken, OToken>,
> judge::JudgeService for JudgeServiceImpl<RToken, OToken, JobApi>
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
