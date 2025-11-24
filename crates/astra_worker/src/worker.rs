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
use wait_timeout::ChildExt;

const MAP_SIZE: usize = 262_144;


pub fn worker(
    id: u16,
    target: PathBuf,
    timeout: u64,
    arguments: Vec<String>,
    recv_input: Receiver<Vec<u8>>,
    send_cov: Sender<(u16, Vec<u8>, Vec<u8>)>,
    send_crash: Sender<bool>,
    send_hang: Sender<bool>,
)
{

    let mut finding = false;
    let mut hang = false;
    println!("worker {id} started");
    let timeout_ms = std::time::Duration::from_millis(timeout);

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
        let mut args = arguments.clone();
        for arg in args.iter_mut() {
            if arg == "@@" {
                *arg = tmp.clone().into_os_string().into_string().unwrap();
            }
        }

        let mut child = Command::new(&target)
            .args(&args)
            .env("ASTRA_SHM_ID", &shm_id)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("Failed to run the target");
        
        let status_code = match child.wait_timeout(timeout_ms).unwrap() {
            Some(status) => status.code(),
                None => {
                    child.kill().unwrap();
                    hang = true;
                    let _ = send_hang.send(hang);
                    child.wait().unwrap().code()

                }
            };
        
        let status =  child.wait().unwrap();
        match status.code() {
            Some(code) =>  {
                match code {
                    0 => {} 
                    _ => {}
                    11 => { 
                        record_crash(input.clone());
                        finding = true;
                        let _ = send_crash.send(finding);
                    }
                }
            }
            None => {}
        }

        let local_copy = edge_map.to_vec();
        send_cov.send((id, input, local_copy)).unwrap();
        clean_shared_memory(ptr, shm_id.as_str());
    }
}