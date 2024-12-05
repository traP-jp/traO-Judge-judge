use crate::container::Container as ContainerTrait;
use std::cmp::{Ord, Ordering, PartialEq, PartialOrd};
use tokio::sync::oneshot::Sender;
use tokio::sync::MutexGuard;
use uuid::Uuid;

pub struct JobAcquisition<'a, ContainerType: ContainerTrait, JobOrderingType: Ord> {
    pub sender: Sender<MutexGuard<'a, ContainerType>>,
    pub ordering: JobOrderingType,
    pub id: Uuid,
}

impl<ContainerType: ContainerTrait, JobOrderingType: Ord> PartialEq
    for JobAcquisition<'_, ContainerType, JobOrderingType>
{
    fn eq(&self, other: &Self) -> bool {
        self.ordering == other.ordering
    }
}

impl<ContainerType: ContainerTrait, JobOrderingType: Ord> Eq
    for JobAcquisition<'_, ContainerType, JobOrderingType>
{
}

impl<ContainerType: ContainerTrait, JobOrderingType: Ord> PartialOrd
    for JobAcquisition<'_, ContainerType, JobOrderingType>
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.ordering.cmp(&other.ordering))
    }
}

impl<ContainerType: ContainerTrait, JobOrderingType: Ord> Ord
    for JobAcquisition<'_, ContainerType, JobOrderingType>
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.ordering.cmp(&other.ordering)
    }
}
