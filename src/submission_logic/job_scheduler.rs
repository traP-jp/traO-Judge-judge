mod acquisition;
use super::job_scheduler::acquisition::JobAcquisition;
use crate::container::Container as ContainerTrait;
use anyhow::{Context, Result};
use std::collections::{BinaryHeap, HashMap};
use std::sync::Arc;
use tokio::sync::oneshot::{self, Receiver, Sender};
use tokio::sync::{Mutex, MutexGuard};
use uuid::Uuid;

pub struct JobScheduler<'a, ContainerType: ContainerTrait, JobOrderType: Ord + Clone> {
    containers: HashMap<Uuid, Arc<Mutex<ContainerType>>>,
    job_acquisition_queue: BinaryHeap<JobAcquisition<'a, ContainerType, JobOrderType>>,
}

impl<'a, ContainerType: ContainerTrait, JobOrderType: Ord + Clone>
    JobScheduler<'a, ContainerType, JobOrderType>
{
    pub fn get_container_waiting_rx(
        &mut self,
        ordering: JobOrderType,
    ) -> Receiver<MutexGuard<'a, ContainerType>> {
        let (tx, rx): (Sender<MutexGuard<'a, ContainerType>>, _) = oneshot::channel();
        let job_acquisition: JobAcquisition<'a, ContainerType, JobOrderType> = JobAcquisition {
            sender: tx,
            ordering,
            id: Uuid::new_v4(),
        };
        self.job_acquisition_queue.push(job_acquisition);
        rx
    }

    pub fn add_container(&mut self, container: ContainerType, id: Uuid) -> Result<()> {
        self.containers.insert(id, Arc::new(Mutex::new(container)));
        Ok(())
    }

    pub fn remove_container(&mut self, id: Uuid) -> Result<()> {
        self.containers
            .remove(&id)
            .context("Failed to remove container")
            .map(|_| ())
    }

    pub async fn distribute(&'a mut self) -> Result<()> {
        let (containers, job_acquisition_queue) =
            (&self.containers, &mut self.job_acquisition_queue);
        let mut available_containers: Vec<(&Uuid, MutexGuard<'a, ContainerType>)> = containers
            .iter()
            .filter_map(|(id, container)| {
                let unlocked_container: Result<MutexGuard<'a, ContainerType>, _> =
                    container.try_lock();
                match unlocked_container {
                    Ok(guard) => Some((id, guard)),
                    Err(_) => None,
                }
            })
            .collect();
        while available_containers.is_empty() {
            let job_acquisition = job_acquisition_queue.pop();
            match job_acquisition {
                None => break,
                Some(job_acquisition) => {
                    let container: (&Uuid, MutexGuard<'a, ContainerType>) =
                        available_containers.pop().unwrap();
                    job_acquisition.sender.send(container.1).map_err(|_| {
                        anyhow::anyhow!("Failed to send container to job acquisition")
                    })?;
                }
            }
        }
        Ok(())
    }
}
