use tokio::sync::oneshot::Sender;
pub struct OneshotSenderWithOrd<PriorityType: Ord, ItemType> {
    pub priority: PriorityType,
    pub _sender: Sender<ItemType>,
}

impl<PriorityType: Ord, ItemType> PartialEq for OneshotSenderWithOrd<PriorityType, ItemType> {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

impl<PriorityType: Ord, ItemType> Eq for OneshotSenderWithOrd<PriorityType, ItemType> {}

impl<PriorityType: Ord, ItemType> PartialOrd for OneshotSenderWithOrd<PriorityType, ItemType> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.priority.cmp(&other.priority))
    }
}

impl<PriorityType: Ord, ItemType> Ord for OneshotSenderWithOrd<PriorityType, ItemType> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.priority.cmp(&other.priority)
    }
}
