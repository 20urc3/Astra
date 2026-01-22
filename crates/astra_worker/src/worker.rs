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
use astra_scheduler::*;
use astra_tui::log_info;

use std::os::unix::process::ExitStatusExt;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use flume::{Receiver, Sender};
use wait_timeout::ChildExt;
use colored_text::Colorize;
use chrono;

const MAP_SIZE: usize = 262_144;


pub fn worker(
    id: u16,
    target: PathBuf,
    timeout: u64,
    args_before: Vec<String>,
    args_after: Vec<String>,
    corpus: InputQueue,
    send_cov: Sender<(u16, Vec<u8>, Vec<u8>)>,
    send_crash: Sender<bool>,
    send_hang: Sender<bool>,
)
 
{

    // Initialize findings/hangs holder and shared memory
    let mut finding = false;
    let mut hang = false;
    log_info!("Astra-worker", "The worker {id} has started");
    let timeout_ms = std::time::Duration::from_millis(timeout);
    let mut local_map = vec![0u8; MAP_SIZE];
    let (_, ptr, shm_id) = create_shared_memory(id);
        let edge_map = unsafe {
            std::slice::from_raw_parts_mut(ptr as *mut u8, MAP_SIZE)
        };


    loop {
        let Some(mut input) = corpus.get_next() else {
            std::thread::sleep(std::time::Duration::from_millis(10));
            continue;
        };

        random_havoc(&mut input);
        edge_map.fill(0);

        let tmp = std::env::temp_dir().join(format!("input_{id}.tmp"));
        std::fs::write(&tmp, &input).unwrap();
        let mut cmd_args = args_before.clone();
        cmd_args.push(tmp.to_string_lossy().to_string());
        cmd_args.extend(args_after.clone());

        let mut child = Command::new(&target)
            .args(&cmd_args)
            .env("ASTRA_SHM_ID", &shm_id)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("Failed to run the target");
        
        match child.wait_timeout(timeout_ms).unwrap() {
            Some(status) => {
                match status.signal() {
                    Some(0) => {}
                    Some(11) => { 
                        record_crash(input.clone());
                        finding = true;
                        let _ = send_crash.send(finding);
                    }
                    _ => {}
                }
            }

            None => {
                child.kill().unwrap();
                hang = true;
                let _ = send_hang.send(hang);
                child.wait().unwrap();

            }
        }; 


        let local_copy = edge_map.to_vec();
        send_cov.send((id, input, local_copy)).unwrap();
    }
}