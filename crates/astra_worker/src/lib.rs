pub mod worker;
mod test;

use worker::*;

use astra_collector::collect_corpus;
use astra_scheduler::testcase_queue::*;

use std::{path::PathBuf, sync::{Arc, Mutex}};
use std::thread;


/// Instantiate the shared objects between threads
pub fn initiate_shared_objects(
    input_dir: PathBuf, 
    target: &PathBuf) {

    // Wrap the global map in mutex for thread safe mutation
    // and Arc for safe shared ownership
    let global_map = Arc::new(Mutex::new(vec![0u8; 262_144]));

    // Create a testcase queue
    let queue = Arc::new(Mutex::new(TQueue::new()));

    // Collect corpus
    let corpus = collect_corpus(&input_dir);
    
    println!("Shared memory objects initiated !");
}

/// Creates and run the worker pool
pub fn running_workers(
    num_thr: u16,
    target: PathBuf)
    
    {
    println!("Hi from running workers");

    thread::scope(|s| {
        for thr_id in 1..num_thr {
            let mut target_copy = target.clone();
            let target_copy = Arc::new(target_copy);
            s.spawn(move || {
                worker(thr_id, target_copy);
           });
        }
    });
}