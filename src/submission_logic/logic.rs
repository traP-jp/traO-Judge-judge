use crate::container::Container as ContainerTrait;
use crate::custom_rc::{FileLink as FileLinkTrait, FileLinkFactory as FileLinkFactoryTrait};
use crate::submission_logic::cmd_input_parser::CmdInput;
use crate::text_resource_repository::TextResourceRepository as TextResourceRepositoryTrait;
use crate::models::{judge_recipe::SubmissionInput, judge_result::SubmissionOutput};
use anyhow::{Context, Result};
use std::collections::HashMap;
use std::time::Duration;
use std::collections::VecDeque;
use std::future::Future;
use std::sync::MutexGuard;
use tokio::sync::oneshot::{self, Receiver, Sender};
use tokio::sync::Mutex;
use uuid::Uuid;
use super::job_scheduler::JobScheduler;
use futures::future::join;
use std::path::PathBuf;

pub struct Logic<
    'a,
    ContainerType: ContainerTrait,
    JobOrderingType: Ord + Clone,
    ExternalAccessKey: Eq + std::hash::Hash + Clone + ToString,
    TextResourceRepositoryType: TextResourceRepositoryTrait<ExternalAccessKey>,
    FileLinkType: FileLinkTrait,
    FileLinkFactoryType: FileLinkFactoryTrait<ExternalAccessKey, TextResourceRepositoryType, FileLinkType>,
> {
    job_scheduler: Mutex<JobScheduler<'a, ContainerType, JobOrderingType>>,
    file_link_factory: FileLinkFactoryType,
    _phantom: std::marker::PhantomData<(ExternalAccessKey, TextResourceRepositoryType, FileLinkType)>,
}

impl <
    'a,
    ContainerType: ContainerTrait,
    JobOrderingType: Ord + Clone,
    ExternalAccessKey: Eq + std::hash::Hash + Clone + ToString,
    TextResourceRepositoryType: TextResourceRepositoryTrait<ExternalAccessKey>,
    FileLinkType: FileLinkTrait,
    FileLinkFactoryType: FileLinkFactoryTrait<ExternalAccessKey, TextResourceRepositoryType, FileLinkType>,
> Logic<'a, ContainerType, JobOrderingType, ExternalAccessKey, TextResourceRepositoryType, FileLinkType, FileLinkFactoryType> {
    pub fn new(
        job_scheduler: JobScheduler<'a, ContainerType, JobOrderingType>,
        file_link_factory: FileLinkFactoryType,
    ) -> Self {
        Self {
            job_scheduler: Mutex::new(job_scheduler),
            file_link_factory,
            _phantom: std::marker::PhantomData,
        }
    }


    async fn create_file_links(&self, cmd_input: CmdInput<ExternalAccessKey>) -> Result<HashMap<Uuid, (PathBuf, FileLinkType)>> {
        let text_file_creating_futures: Vec<_> = cmd_input.file_links
            .iter()
            .filter_map(|(path, recipe)| {
                let file_link_factory = &self.file_link_factory;
                match recipe {
                    super::cmd_input_parser::FileLinkRecipe::TextFile(text_resource_id, replica, id) => {
                        Some(async move {
                            (
                                id.clone(),
                                path.clone(),
                                file_link_factory
                                    .get_text_file_links(text_resource_id.clone(), *replica)
                                    .await,
                            )
                        })
                    }
                    super::cmd_input_parser::FileLinkRecipe::Directory(_) => {
                        None
                    }
                }
            })
            .collect();
        let future = futures::future::join_all(text_file_creating_futures);
        let file_links = future.await
            .into_iter()
            .collect::<Vec<_>>();
        let mut file_links_map = HashMap::new();
        for (id, path, file_link) in file_links {
            let file_link_vec = file_link.context("Failed to create file link")?;
            for file_link in file_link_vec {
                file_links_map.insert(id.clone(), (path.clone(), file_link));
            }
        }
        Ok(file_links_map)
    }

    async fn create_directories(&self, cmd_input: CmdInput<ExternalAccessKey>) -> Result<HashMap<Uuid, (PathBuf, FileLinkType)>> {
        let directory_creating_futures: Vec<_> = cmd_input.file_links
            .iter()
            .filter_map(|(path, recipe)| {
                let file_link_factory = &self.file_link_factory;
                match recipe {
                    super::cmd_input_parser::FileLinkRecipe::TextFile(_, _, _) => {
                        None
                    }
                    super::cmd_input_parser::FileLinkRecipe::Directory(id) => {
                        Some(async move {
                            (
                                id.clone(),
                                path.clone(),
                                file_link_factory
                                    .get_directory_link()
                                    .await,
                            )
                        })
                    }
                }
            })
            .collect();
        let future = futures::future::join_all(directory_creating_futures);
        let file_links = future.await
            .into_iter()
            .collect::<Vec<_>>();
        let mut file_links_map = HashMap::new();
        for (id, path, file_link) in file_links {
            let file_link_vec = file_link.context("Failed to create file link")?;
            file_links_map.insert(id.clone(), (path.clone(), file_link_vec));
        }
        Ok(file_links_map)
    }
}

impl <
    'a,
    ContainerType: ContainerTrait,
    JobOrderingType: Ord + Clone,
    ExternalAccessKey: Eq + std::hash::Hash + Clone + ToString,
    TextResourceRepositoryType: TextResourceRepositoryTrait<ExternalAccessKey>,
    FileLinkType: FileLinkTrait,
    FileLinkFactoryType: FileLinkFactoryTrait<ExternalAccessKey, TextResourceRepositoryType, FileLinkType>,
> 
super::Logic<ContainerType, JobOrderingType>
for Logic<
    'a,
    ContainerType,
    JobOrderingType,
    ExternalAccessKey,
    TextResourceRepositoryType,
    FileLinkType,
    FileLinkFactoryType,
> {
    async fn add_container(&self, id: Uuid, container: ContainerType) -> Result<()> {
        let mut job_scheduler = self.job_scheduler.lock().await;
        job_scheduler.add_container(container, id)
            .context("Failed to add container")
    }

    async fn release_container(&self, id: Uuid) -> Result<()> {
        let mut job_scheduler = self.job_scheduler.lock().await;
        job_scheduler.remove_container(id)
            .context("Failed to remove container")
    }

    async fn exec(&self, sub_input: SubmissionInput<JobOrderingType>, connection_time_limit: Duration, execution_time_limit: Duration) -> Result<SubmissionOutput> {
        let container_rx;
        {
            let order = sub_input.job_order.clone();
            let mut job_scheduler = self.job_scheduler.lock().await;
            container_rx = job_scheduler.get_container_waiting_rx(order);
        }
        let mut container = container_rx.await
            .context("Failed to acquire container")?;
        let cmd_input: CmdInput<ExternalAccessKey> = super::cmd_input_parser::get_cmd_input(&sub_input);
        let file_links = join(
            self.create_file_links(cmd_input.clone()),
            self.create_directories(cmd_input.clone()),
        ).await;
        let (file_links, directories) = file_links;
        let file_links = file_links?;
        let directories = directories?;
        let merged_file_links = file_links.into_iter()
            .chain(directories.into_iter())
            .collect::<HashMap<Uuid, (PathBuf, FileLinkType)>>();
        unimplemented!()
    }

}