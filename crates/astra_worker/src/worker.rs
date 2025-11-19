//! This file provides function to spawn a worker.
//! A worker is thread that spawn the target program as a child-process
//! 
//! Its arguments are:
//! - thread id
//! - A input to process
//! - A path to the target program
//! 
//! - It establish a unique shared memory between the thread 
//! and the child-process, used to share the edge_map
//! - It returns input and new edge-map to the main process

use astra_observer::shm::*;
use astra_mutator::*;

use std::path::PathBuf;
use std::process::Command;
use crossbeam::channel::Receiver;
use crossbeam_channel::Sender;

const MAP_SIZE: usize = 262_144;

pub fn worker(
    id: u16,
    target: PathBuf,
    recv_input: Receiver<Vec<u8>>,
    send_cov: Sender<(u16, Vec<u8>, Vec<u8>)>,
)
{
    // Todo: The worker need to send the input to the main function
    // The main function then needs to check if the map is better or not
    // If yes it adds to interesting corpus if not it adds to normal corpus
    
    println!("worker {id} started");
    loop {
        let original = recv_input.recv().unwrap();

        // mutate a copy, not the original
        let mut mutated = original.clone();
        random_havoc(&mut mutated);

        let (_, ptr, shm_id) = create_shared_memory(id);
        let edge_map = unsafe {
            std::slice::from_raw_parts_mut(ptr as *mut u8, MAP_SIZE)
        };
        edge_map.fill(0);

        let tmp = std::env::temp_dir().join(format!("input_{id}.tmp"));
        std::fs::write(&tmp, &mutated).unwrap();

        Command::new(&target)
            .arg(&tmp)
            .env("ASTRA_THR_ID", id.to_string())
            .spawn()
            .unwrap()
            .wait()
            .unwrap();

        let local_copy = edge_map.to_vec();
        send_cov.send((id, mutated, local_copy)).unwrap();
        clean_shared_memory(ptr, shm_id.as_str());
    }
}