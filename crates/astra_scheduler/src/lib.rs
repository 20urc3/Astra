use flume::{Receiver, unbounded, Sender};

struct Corpus {
    normal: Sender<Vec<u8>>,
    priority: Sender<Vec<u8>>,
    normal_rx: Receiver<Vec<u8>>,
    priority_rx: Receiver<Vec<u8>>,
}

impl Corpus {
    fn new() -> Self {
        let (normal, normal_rx) = unbounded();
        let (priority, priority_rx) = unbounded();
        Self { normal, priority, normal_rx, priority_rx }
    }
    
    fn get_next(&self) -> Option<Vec<u8>> {
        self.priority_rx.try_recv().ok()
            .or_else(|| self.normal_rx.try_recv().ok())
    }
    
    fn add_normal(&self, input: Vec<u8>) {
        let _ = self.normal.send(input);
    }
    
    fn add_priority(&self, input: Vec<u8>) {
        let _ = self.priority.send(input);
    }
}