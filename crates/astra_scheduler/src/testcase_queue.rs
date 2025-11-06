//! This implement a unbounded Queue for testcases 

use std::sync::{Arc, Mutex};

pub struct TQueue {
    pub queue_list: Arc<Mutex<Vec<usize>>>,
}

impl TQueue {
    pub fn new() -> Self {
        Self {
            queue_list: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn push(&self, i: usize) {
        self.queue_list.lock().unwrap().push(i);
    }

    pub fn pop(&self) -> Option<usize> {
        self.queue_list.lock().unwrap().pop()
    }
}
