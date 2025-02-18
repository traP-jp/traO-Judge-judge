pub mod registry_client;
pub mod registry_server;

pub fn new_registry() -> (
    registry_server::RegistryServer,
    registry_client::RegistryClient,
) {
    let registry = std::sync::Arc::new(tokio::sync::Mutex::new(std::collections::HashMap::new()));
    let server = registry_server::RegistryServer {
        registry: registry.clone(),
        dep_id_to_name: std::sync::Arc::new(tokio::sync::Mutex::new(std::collections::HashMap::new())),
    };
    let client = registry_client::RegistryClient { registry };
    (server, client)
}
