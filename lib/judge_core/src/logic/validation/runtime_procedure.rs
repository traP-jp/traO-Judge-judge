fn validate(procedure: &crate::model::procedure::runtime::Procedure) -> anyhow::Result<bool> {
    let mut edges = vec![];
    for execution in &procedure.executions {
        for dependency in &execution.dependencies {
            edges.push(super::has_cycle::Edge {
                from: dependency.runtime_id,
                to: execution.runtime_id,
            });
        }
    }
    super::has_cycle::has_cycle(edges)
        .map_err(|e| anyhow::anyhow!("Failed to validate runtime procedure: {}", e))
}
