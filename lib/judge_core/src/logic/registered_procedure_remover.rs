use crate::model::problem_registry::{ProblemRegistryServer, RemovalError};
use crate::model::procedure::registered::*;
use futures::future::join_all;

pub async fn remove<PRServer: ProblemRegistryServer>(
    procedure: Procedure,
    pr_server: PRServer,
) -> Result<(), RemovalError> {
    let mut futures = Vec::new();
    for text in procedure.texts.iter() {
        futures.push(pr_server.remove(text.resource_id.clone()));
    }
    let future = join_all(futures);
    future
        .await
        .into_iter()
        .collect::<Result<Vec<_>, _>>()
        .map(|_| ())
}
