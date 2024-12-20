use std::collections::BinaryHeap;
mod sender_with_ord;
use sender_with_ord::OneshotSenderWithOrd;
use tokio::sync::Mutex;
use std::sync::Arc;


/// single-producer multi consumer oneshot channel
pub struct Spmc<PriorityType: Ord, ItemType> {
    heap: BinaryHeap<OneshotSenderWithOrd<PriorityType, ItemType>>,
    _phantom: std::marker::PhantomData<ItemType>,
}


impl <
    PriorityType: Ord,
    ItemType,
> Spmc<PriorityType, ItemType> {
    fn new() -> Self {
        Self {
            heap: BinaryHeap::new(),
            _phantom: std::marker::PhantomData,
        }
    }
}


/// sender of single-producer multi consumer oneshot channel
pub struct SpmcSender<PriorityType: Ord, ItemType> {
    channel: Arc<Mutex<Spmc<PriorityType, ItemType>>>,
}

impl <
    PriorityType: Ord,
    ItemType,
> SpmcSender<PriorityType, ItemType> {
    async fn send(&self, item: ItemType) -> Result<(), ItemType> {
        let sender = {
            let mut heap = self.channel.lock().await;
            match heap.heap.pop() {
                Some(OneshotSenderWithOrd { sender: top_sender, .. }) => {
                    Some(top_sender)
                },
                None => None,
            }
        };
        match sender {
            Some(sender) => {
                sender.send(item)
            },
            None => {
                Err(item)
            },
        }
    }
}

/// receiver factory of single-producer multi consumer oneshot channel
pub struct SpmcReceiverFactory<PriorityType: Ord, ItemType> {
    channel: Arc<Mutex<Spmc<PriorityType, ItemType>>>,
}

impl <
    PriorityType: Ord,
    ItemType,
> SpmcReceiverFactory<PriorityType, ItemType> {
    async fn new_receiver(&self, priority: PriorityType) -> SpmcReceiver<ItemType> {
        let (sender, receiver) = tokio::sync::oneshot::channel();
        let sender_with_ord = OneshotSenderWithOrd {
            priority,
            sender,
        };
        let mut heap = self.channel.lock().await;
        heap.heap.push(sender_with_ord);
        receiver
    }
}

/// receiver of single-producer multi consumer oneshot channel
pub type SpmcReceiver<ItemType> = tokio::sync::oneshot::Receiver<ItemType>;


pub fn channel<PriorityType: Ord, ItemType>() -> (SpmcSender<PriorityType, ItemType>, SpmcReceiverFactory<PriorityType, ItemType>) {
    let channel = Arc::new(Mutex::new(Spmc::new()));
    (SpmcSender { channel: channel.clone() }, SpmcReceiverFactory { channel })
}