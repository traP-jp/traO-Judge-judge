use crate::model::procedure::writer_schema::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Job {
    Resource(ResourceKind),
    Execution(Execution),
    Script(Text),
}

/// Internal builder to create a judge-procedure
#[derive(Debug, Clone)]
pub struct ProcedureBuilder {
    jobs: HashMap<String, Job>,
}

impl From<ProcedureBuilder> for Procedure {
    fn from(builder: ProcedureBuilder) -> Self {
        Self {
            resources: builder
                .jobs
                .iter()
                .filter_map(|(_, job)| match job {
                    Job::Resource(resource) => Some(resource.clone()),
                    _ => None,
                })
                .collect(),
            executions: builder
                .jobs
                .iter()
                .filter_map(|(_, job)| match job {
                    Job::Execution(execution) => Some(execution.clone()),
                    _ => None,
                })
                .collect(),
            scripts: builder
                .jobs
                .iter()
                .filter_map(|(_, job)| match job {
                    Job::Script(script) => Some(script.clone()),
                    _ => None,
                })
                .collect(),
        }
    }
}

impl ProcedureBuilder {
    pub fn new() -> Self {
        ProcedureBuilder {
            jobs: HashMap::new(),
        }
    }

    /// Add a resource to the procedure and return the name of the resource
    pub fn add_resource(&mut self, resource: ResourceKind) -> Result<String, AddJobError> {
        let name = match &resource {
            ResourceKind::EmptyDirectory(empty_directory) => empty_directory.name.clone(),
            ResourceKind::RuntimeTextFile(runtime_text) => runtime_text.name.clone(),
            ResourceKind::TextFile(text) => text.name.clone(),
        };
        self.jobs
            .insert(name.clone(), Job::Resource(resource))
            .map_or_else(
                || Ok(name.clone()),
                |_| Err(AddJobError::ResourceAlreadyExists(name.clone())),
            )?;
        Ok(name)
    }

    /// Add a script to the procedure and return the name of the script
    pub fn add_script(&mut self, script: Text) -> Result<String, AddJobError> {
        let name = script.name.clone();
        self.jobs
            .insert(name.clone(), Job::Script(script))
            .map_or_else(
                || Ok(name.clone()),
                |_| Err(AddJobError::ResourceAlreadyExists(name.clone())),
            )?;
        Ok(name)
    }

    /// Add an execution to the procedure and return the name of the execution
    pub fn add_execution(&mut self, execution: Execution) -> Result<String, AddJobError> {
        let name = execution.name.clone();
        // Check if all dependencies are present
        for dep in execution.dependency.iter() {
            self.jobs
                .get(&dep.ref_to)
                .ok_or(AddJobError::DependencyNotFound(dep.ref_to.clone()))?;
        }
        self.jobs
            .get(&execution.script_name)
            .ok_or(AddJobError::DependencyNotFound(
                execution.script_name.clone(),
            ))?;
        // Insert the execution
        let _ = self
            .jobs
            .insert(name.clone(), Job::Execution(execution))
            .map_or_else(
                || Ok(name.clone()),
                |_| Err(AddJobError::ResourceAlreadyExists(name.clone())),
            )?;
        Ok(name)
    }

    /// Export the procedure
    pub fn get_procedure(&self) -> Procedure {
        Procedure {
            resources: self
                .jobs
                .iter()
                .filter_map(|(_, job)| match job {
                    Job::Resource(resource) => Some(resource.clone()),
                    _ => None,
                })
                .collect(),
            executions: self
                .jobs
                .iter()
                .filter_map(|(_, job)| match job {
                    Job::Execution(execution) => Some(execution.clone()),
                    _ => None,
                })
                .collect(),
            scripts: self
                .jobs
                .iter()
                .filter_map(|(_, job)| match job {
                    Job::Script(script) => Some(script.clone()),
                    _ => None,
                })
                .collect(),
        }
    }
}

#[derive(Clone, Debug, thiserror::Error)]
pub enum AddJobError {
    #[error("Name {0} already exists")]
    ResourceAlreadyExists(String),
    #[error("Dependency {0} not found")]
    DependencyNotFound(String),
}

#[derive(Clone, Debug, thiserror::Error)]
pub enum SchemaSerializationError {
    #[error("Failed to serialize schema: {0}")]
    SerializationError(String),
}
