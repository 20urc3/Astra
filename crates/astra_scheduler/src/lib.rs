use flume::{Receiver, unbounded, Sender};

type Corpus = Vec<Vec<u8>>;

#[derive(Clone)]
pub struct InputsQueue {
    send_normal_queue: Sender<Corpus>,
    send_priority_queue: Sender<Corpus>,
    recv_normal_queue: Receiver<Corpus>,
    recv_priority_queue: Receiver<Corpus>,
}

impl InputsQueue {
    pub fn new() -> Self {

        let (send_normal_queue, recv_normal_queue) = unbounded();
        let (send_priority_queue, recv_priority_queue) = unbounded();

        Self { 
            send_normal_queue,
            send_priority_queue,
            recv_normal_queue,
            recv_priority_queue 
        }
    }
    
    pub fn get_next(&self) -> Option<Corpus> {
        self.recv_priority_queue.try_recv().ok()
            .or_else(|| self.recv_normal_queue.try_recv().ok())
    }
    
    pub fn add_normal(&self, Corpus: Corpus) {
        let _ = self.send_normal_queue.send(Corpus);
    }
    
    pub fn add_priority(&self, Corpus: Corpus) {
        let _ = self.send_priority_queue.send(Corpus);
    }
}