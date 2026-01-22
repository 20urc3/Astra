use flume::{Receiver, unbounded, Sender};

type Corpus = Vec<Input>;
type Input = Vec<u8>;

#[derive(Clone)]
pub struct CorpusQueue {
    send_normal_queue: Sender<Corpus>,
    send_priority_queue: Sender<Corpus>,
    recv_normal_queue: Receiver<Corpus>,
    recv_priority_queue: Receiver<Corpus>,
}

impl CorpusQueue {
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
    
    pub fn add_normal(&self, corpus: Corpus) {
        let _ = self.send_normal_queue.send(corpus);
    }
    
    pub fn add_priority(&self, corpus: Corpus) {
        let _ = self.send_priority_queue.send(corpus);
    }
}

#[derive(Clone)]
pub struct InputQueue {
    send_normal_queue: Sender<Input>,
    send_priority_queue: Sender<Input>,
    recv_normal_queue: Receiver<Input>,
    recv_priority_queue: Receiver<Input>,
}

impl InputQueue {
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
    
    pub fn get_next(&self) -> Option<Input> {
        self.recv_priority_queue.try_recv().ok()
            .or_else(|| self.recv_normal_queue.try_recv().ok())
    }
    
    pub fn add_normal(&self, input: Input) {
        let _ = self.send_normal_queue.send(input);
    }
    
    pub fn add_priority(&self, input: Input) {
        let _ = self.send_priority_queue.send(input);
    }
}

