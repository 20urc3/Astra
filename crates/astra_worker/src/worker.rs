//! This file provides function to spawn a worker.
//! A worker is thread that spawn the target program as a child-process
//! 
//! Its arguments are:
//! - thread id
//! - the index of the input to process
//! - a reference to a shared corpus 
//! - a reference to a global map shared between workers
//! - the path of the target program
//! 
//! - It establish a unique shared memory between the thread 
//! and the child-process, used to shared the edge_map
//! - It is responsible to add interesting input returned by the child process 
//! to the corpus and the scheduler

use astra_observer::{coverage::*, shm::*};

use std::io::Write;
use std::path::PathBuf;
use std::sync::{Mutex, Arc};
use std::process::{Command, Stdio};
use std::fs::File;

const MAP_SIZE: usize = 262_144;
const SYNC_INTERVAL: usize = 1337;

pub fn worker(thr_id: u16, target: Arc<PathBuf>) { 
    println!("Hi from worker: {}", thr_id);
    // Local coverage map
    let mut local_map = vec![0u8; MAP_SIZE];
    // Periodically sync to global map

    // Create shared memory for the child process
    let (fd, ptr, shm_id) = create_shared_memory(thr_id);

    // Map the shared memory edge_map
    let edge_map: &mut [u8] = unsafe { std::slice::from_raw_parts_mut(ptr as *mut u8, MAP_SIZE) };

    // Pick one input

    // Clear edge_map
    edge_map.fill(0);
    local_map.fill(0);

    // Write input to a temporary file


    // Spawn target program
    let mut n = 0;
    while n < 10 {
        let mut child = Command::new(target.as_path())
            .arg("docs/writing_astra/mini_corpus/testcase1")
            .env("ASTRA_THR_ID", thr_id.to_string())
            .spawn()
            .expect("Couldn't spawn the child process");

        // Wait for the child to finish
        let _ = child.wait().expect("Failed to wait for child process");
        
        // Compare coverage to global map
        let flags = compare_global_to_edge(&edge_map, &mut local_map);
        if flags.new_edge { println!("[NEW_EDGE] - The edge_map contains new edge(s)."); }
        if flags.new_hit  { println!("[NEW_HIT] - The hit-count of some edge has increased."); }
        else {
            println!("No new edge or hitcount for this run");
        }

        local_map = populate_global_map(&edge_map);
        edge_map.fill(0);
        n +=1;
    }

    // Clean shared memory
    clean_shared_memory(ptr, shm_id.as_str());

}