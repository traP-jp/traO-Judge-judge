use std::sync::Arc;

use domain::repository::resource_id_counter::ResourceIdCounterRepository;
use infra::provider::Provider;
use judge_core::model::{identifiers::ResourceId, problem_registry::ProblemRegistryServer};
use tokio_cron_scheduler::{Job, JobScheduler};

pub async fn init_scheduler(provider: &Provider) -> anyhow::Result<()> {
    let sched = JobScheduler::new().await?;

    let resource_id_counter_repo = Arc::new(provider.provide_resource_id_counter_repository());
    let problem_registry_server = Arc::new(provider.provide_problem_registry_server());

    let job = Job::new_async("0 */1 * * * *", move |_uuid, _l| {
        let resource_id_counter_repo = Arc::clone(&resource_id_counter_repo);
        let problem_registry_server = Arc::clone(&problem_registry_server);

        Box::pin(async move {
            let deletable_ids = resource_id_counter_repo
                .get_deletable_resource_ids(10)
                .await
                .unwrap_or_else(|e| {
                    tracing::error!("Failed to get deletable resource IDs: {}", e);
                    vec![]
                });

            let mut deleted = vec![];
            let mut failed = vec![];

            for uuid in deletable_ids {
                let resource_id = uuid;
                if let Err(e) = problem_registry_server.remove(resource_id.into()).await {
                    tracing::error!(
                        "Failed to delete resource {} from registry server: {}",
                        uuid,
                        e
                    );
                    failed.push(uuid);
                } else {
                    deleted.push(uuid);
                }
            }

            resource_id_counter_repo
                .delete_resource_ids(deleted.clone())
                .await
                .unwrap_or_else(|e| {
                    tracing::error!("Failed to delete resource IDs from DB: {}", e);
                });

            resource_id_counter_repo
                .update_timestamp_ids(failed.clone())
                .await
                .unwrap_or_else(|e| {
                    tracing::error!(
                        "Failed to update timestamp for failed resource IDs in DB: {}",
                        e
                    );
                });
        })
    })?;

    sched.add(job).await?;
    sched.start().await?;

    Ok(())
}
