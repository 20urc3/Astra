
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug)]
pub enum MutationType {
    ByteInsertion,
    ByteDeletion,
    ByteSwap,
}

pub struct MutationQueue {
    pub queue_list: Arc<Mutex<Vec<MutationType>>>,
}

impl MutationQueue {
    pub fn new() -> Self {
        Self {
            queue_list: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn push(&self, m: MutationType) {
        self.queue_list.lock().unwrap().push(m);
    }

    pub fn pop(&self) -> Option<MutationType> {
        self.queue_list.lock().unwrap().pop()
    }
}
