mod models;
use models::*;

use super::job_scheduler::JobScheduler;
use crate::container::Container as ContainerTrait;
use crate::custom_rc::{
    file_link, FileLink as FileLinkTrait, FileLinkFactory as FileLinkFactoryTrait,
    SymlinkLink as SymlinkLinkTrait,
};
use crate::models::{judge_recipe::SubmissionInput, judge_result::SubmissionOutput};
use crate::remote_exec::ExecutionOutput;
use crate::submission_logic::cmd_input_parser::{get_cmd_input, models::*};
use crate::text_resource_repository::TextResourceRepository as TextResourceRepositoryTrait;
use anyhow::{Context, Result};
use futures::future::join;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::future::Future;
use std::path::PathBuf;
use std::time::Duration;
use tokio::sync::oneshot::{self, Receiver, Sender};
use tokio::sync::Mutex;
use tokio::sync::MutexGuard;
use uuid::Uuid;

pub struct Logic<
    'a,
    ContainerType: ContainerTrait,
    JobOrderingType: Ord + Clone,
    ExternalAccessKey: Eq + std::hash::Hash + Clone + ToString,
    TextResourceRepositoryType: TextResourceRepositoryTrait<ExternalAccessKey>,
    FileLinkType: FileLinkTrait,
    SymlinkLinkType: SymlinkLinkTrait<'a, FileLinkType>,
    FileLinkFactoryType: FileLinkFactoryTrait<ExternalAccessKey, TextResourceRepositoryType, FileLinkType>,
> {
    job_scheduler: Mutex<JobScheduler<'a, ContainerType, JobOrderingType>>,
    file_link_factory: FileLinkFactoryType,
    shell_command: String,
    _phantom: std::marker::PhantomData<(
        ExternalAccessKey,
        TextResourceRepositoryType,
        FileLinkType,
        SymlinkLinkType,
    )>,
}

impl<
        'a,
        ContainerType: ContainerTrait,
        JobOrderingType: Ord + Clone,
        ExternalAccessKey: Eq + std::hash::Hash + Clone + ToString,
        TextResourceRepositoryType: TextResourceRepositoryTrait<ExternalAccessKey>,
        FileLinkType: FileLinkTrait,
        SymlinkLinkType: SymlinkLinkTrait<'a, FileLinkType>,
        FileLinkFactoryType: FileLinkFactoryTrait<ExternalAccessKey, TextResourceRepositoryType, FileLinkType>,
    >
    Logic<
        'a,
        ContainerType,
        JobOrderingType,
        ExternalAccessKey,
        TextResourceRepositoryType,
        FileLinkType,
        SymlinkLinkType,
        FileLinkFactoryType,
    >
{
    pub fn new(
        job_scheduler: JobScheduler<'a, ContainerType, JobOrderingType>,
        file_link_factory: FileLinkFactoryType,
        shell_command: String,
    ) -> Self {
        Self {
            job_scheduler: Mutex::new(job_scheduler),
            file_link_factory,
            shell_command,
            _phantom: std::marker::PhantomData,
        }
    }

    async fn create_text_file_links(
        &self,
        cmd_input: CmdInput<ExternalAccessKey>,
    ) -> Result<HashMap<Uuid, (PathBuf, FileLinkType)>> {
        let text_file_creating_futures: Vec<_> = cmd_input
            .file_links
            .iter()
            .filter_map(|(id, recipe)| {
                let file_link_factory = &self.file_link_factory;
                match recipe {
                    FileLinkRecipe::TextFile(recipe) => {
                        let text_resource_id = recipe.text_resource_id.clone();
                        let replica = recipe.replica;
                        let cache = recipe.cache;
                        let path = recipe.path.clone();
                        Some(async move {
                            (
                                id.clone(),
                                path.clone(),
                                file_link_factory
                                    .get_text_file_links(text_resource_id.clone(), replica, cache)
                                    .await,
                            )
                        })
                    }
                    FileLinkRecipe::Directory(_) => None,
                }
            })
            .collect();
        let future = futures::future::join_all(text_file_creating_futures);
        let file_links = future.await.into_iter().collect::<Vec<_>>();
        let mut file_links_map = HashMap::new();
        for (id, path, file_link) in file_links {
            let file_link_vec = file_link.context("Failed to create file link")?;
            for file_link in file_link_vec {
                file_links_map.insert(id.clone(), (path.clone(), file_link));
            }
        }
        Ok(file_links_map)
    }

    async fn create_directory_link(
        &self,
        cmd_input: CmdInput<ExternalAccessKey>,
    ) -> Result<HashMap<Uuid, (PathBuf, FileLinkType)>> {
        let directory_creating_futures: Vec<_> = cmd_input
            .file_links
            .iter()
            .filter_map(|(id, recipe)| {
                let file_link_factory = &self.file_link_factory;
                match recipe {
                    FileLinkRecipe::TextFile(_) => None,
                    FileLinkRecipe::Directory(path) => Some(async move {
                        (
                            id.clone(),
                            path.path.clone(),
                            file_link_factory.get_directory_link().await,
                        )
                    }),
                }
            })
            .collect();
        let future = futures::future::join_all(directory_creating_futures);
        let file_links = future.await.into_iter().collect::<Vec<_>>();
        let mut file_links_map = HashMap::new();
        for (id, path, file_link) in file_links {
            let file_link_vec = file_link.context("Failed to create file link")?;
            file_links_map.insert(id.clone(), (path.clone(), file_link_vec));
        }
        Ok(file_links_map)
    }

    async fn create_file_links(
        &self,
        cmd_input: CmdInput<ExternalAccessKey>,
    ) -> Result<HashMap<Uuid, (PathBuf, FileLinkType)>> {
        let text_file_links = self.create_text_file_links(cmd_input.clone());
        let directory_link = self.create_directory_link(cmd_input.clone());
        let (text_file_links, directory_link) = join(text_file_links, directory_link).await;
        let text_file_links = text_file_links?;
        let directory_link = directory_link?;
        let merged_file_links = text_file_links
            .into_iter()
            .chain(directory_link.into_iter())
            .collect::<HashMap<Uuid, (PathBuf, FileLinkType)>>();
        Ok(merged_file_links)
    }

    async fn single_exec(
        &self,
        single_exec_args: SingleExecutionArgs<'a, FileLinkType, JobOrderingType>,
    ) -> Result<ExecutionOutput> {
        let container_rx;
        {
            let order = single_exec_args.job_order.clone();
            let mut job_scheduler = self.job_scheduler.lock().await;
            container_rx = job_scheduler.get_container_waiting_rx(order);
        }
        let container = container_rx.await.context("Failed to acquire container")?;
        let result = container
            .execute::<FileLinkType, SymlinkLinkType>(
                &single_exec_args.cmd,
                single_exec_args.envs,
                single_exec_args.connection_time_limit,
                single_exec_args.execution_time_limit,
                single_exec_args.file_links,
            )
            .await
            .context("Failed to execute command");
        result
    }
}

impl<
        'a,
        ContainerType: ContainerTrait,
        JobOrderingType: Ord + Clone,
        ExternalAccessKey: Eq + std::hash::Hash + Clone + ToString,
        TextResourceRepositoryType: TextResourceRepositoryTrait<ExternalAccessKey>,
        FileLinkType: FileLinkTrait,
        SymlinkLinkType: SymlinkLinkTrait<'a, FileLinkType>,
        FileLinkFactoryType: FileLinkFactoryTrait<ExternalAccessKey, TextResourceRepositoryType, FileLinkType>,
    > super::Logic<ContainerType, JobOrderingType>
    for Logic<
        'a,
        ContainerType,
        JobOrderingType,
        ExternalAccessKey,
        TextResourceRepositoryType,
        FileLinkType,
        SymlinkLinkType,
        FileLinkFactoryType,
    >
{
    async fn add_container(&self, id: Uuid, container: ContainerType) -> Result<()> {
        let mut job_scheduler = self.job_scheduler.lock().await;
        job_scheduler
            .add_container(container, id)
            .context("Failed to add container")
    }

    async fn release_container(&self, id: Uuid) -> Result<()> {
        let mut job_scheduler = self.job_scheduler.lock().await;
        job_scheduler
            .remove_container(id)
            .context("Failed to remove container")
    }

    async fn exec(
        &self,
        sub_input: SubmissionInput<JobOrderingType>,
        connection_time_limit: Duration,
        execution_time_limit: Duration,
    ) -> Result<SubmissionOutput> {
        let cmd_input: CmdInput<ExternalAccessKey> =
            super::cmd_input_parser::get_cmd_input(&sub_input);
        let file_links = self.create_file_links(cmd_input.clone()).await?;
        unimplemented!()
    }
}
