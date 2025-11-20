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
use astra_monitor::*;

use std::path::PathBuf;
use std::process::{Command, Stdio};
use crossbeam::channel::Receiver;
use crossbeam_channel::Sender;

const MAP_SIZE: usize = 262_144;

pub fn worker(
    id: u16,
    target: PathBuf,
    recv_input: Receiver<Vec<u8>>,
    send_cov: Sender<(u16, Vec<u8>, Vec<u8>)>,
    send_finding: Sender<bool>,
)
{
    // Todo: The worker need to send the input to the main function
    // The main function then needs to check if the map is better or not
    // If yes it adds to interesting corpus if not it adds to normal corpus
    let mut finding = false;
    println!("worker {id} started");
    loop {
        let mut input = recv_input.recv().unwrap();

        random_havoc(&mut input);

        let (_, ptr, shm_id) = create_shared_memory(id);
        let edge_map = unsafe {
            std::slice::from_raw_parts_mut(ptr as *mut u8, MAP_SIZE)
        };
        edge_map.fill(0);

        let tmp = std::env::temp_dir().join(format!("input_{id}.tmp"));
        std::fs::write(&tmp, &input).unwrap();
        

        let status = Command::new(&target)
            .arg(&tmp)
            .arg("-D")
            .env("ASTRA_THR_ID", id.to_string())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .expect("Failed to run the target");

        match status.code() {
            Some(code) =>  {
                match code {
                    0 => {} 
                    _ => {}
                    11 => { 
                        record_crash(input.clone());
                        finding = true;
                        let _ = send_finding.send(finding);
                    }
                }
            }
            None => println!("Process terminated by user signal")
        }
        
        let local_copy = edge_map.to_vec();
        send_cov.send((id, input, local_copy)).unwrap();
        clean_shared_memory(ptr, shm_id.as_str());
    }
}