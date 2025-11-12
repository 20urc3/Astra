use std::sync::{Arc, Mutex};
use std::thread;
use std::collections::VecDeque;

struct TaskQueue {
    tasks: VecDeque<String>,
}

impl TaskQueue {
    fn new() -> Self {
        TaskQueue { tasks: VecDeque::new() }
    }
    fn add_task(&mut self, task: String) {
        self.tasks.push_back(task);
    }
    fn next_task(&mut self) -> Option<String> {
        self.tasks.pop_front()
    }
}

fn main() {
    let queue = Arc::new(Mutex::new(TaskQueue::new()));
    let mut handles = vec![];

    // Add tasks
    {
        let mut queue = queue.lock().unwrap();
        queue.add_task("Task 1".to_string());
        queue.add_task("Task 2".to_string());
        queue.add_task("Task 3".to_string());
    }

    // Workers
    for i in 0..2 {
        let queue = Arc::clone(&queue);
        handles.push(thread::spawn(move || {
            loop {
                let task = {
                    let mut queue = queue.lock().unwrap();
                    queue.next_task()
                };
                match task {
                    Some(task) => {
                        println!("Worker {} processing: {}", i, task);
                        thread::sleep(std::time::Duration::from_millis(500));
                    }
                    None => break,
                }
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}