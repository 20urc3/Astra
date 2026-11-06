//! This file provides function to spawn a worker.
//! A worker is thread that spawn the target program as a child-process
//!
//! Its arguments are:
//! - thread id
//! - A input to process
//! - A path to the target program
//!
//! - It establish a unique shared memory between the thread
//! and the child-process, used to share the program_map
//! - It returns input and new edge-map to the main process

use crate::coverage::{coverage::*, shm::*};
use crate::files::sync_corpus;
use crate::log_info;
use crate::monitor::*;
use crate::mutators::*;
use crate::ui::cli::TargetArgs;
use std::collections::HashSet;
use std::time::Instant;

use chrono;
use colored_text::Colorize;
use flume::Sender;
use std::os::unix::process::ExitStatusExt;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use wait_timeout::ChildExt;

use super::FuzzingStats;

use crate::coverage::MAP_SIZE;

pub fn worker(
    id: u16,
    output_folder_path: PathBuf,
    node_folder: PathBuf,
    mut node_pool: Vec<Vec<u8>>,
    mut file_index: usize,
    target: PathBuf,
    timeout: u64,
    target_args: TargetArgs,
    stats: Arc<FuzzingStats>,
    send_cov: Sender<(u16, Vec<u8>)>,
    send_crash: Sender<bool>,
    send_hang: Sender<bool>,
    cov_mode: String,
    stop: Arc<AtomicBool>,
    sync: bool,
) {
    // TUI info
    log_info!(
        "Astra-worker",
        "The worker {id} has started with {} inputs",
        node_pool.len()
    );
    let timeout_ms = std::time::Duration::from_millis(timeout);

    let mut node_map = vec![0u8; MAP_SIZE];
    let (_, ptr, shm_id) = match create_shared_memory(id) {
        Ok(triple) => triple,
        Err(e) => {
            eprintln!("could not create shared memory for thread {id}: {e}");
            return;
        }
    };

    let program_map = unsafe { std::slice::from_raw_parts_mut(ptr as *mut u8, MAP_SIZE) };

    // Corpus-sync state: which sibling files we've already imported, and when
    // we last synced. Only used when `--sync` is enabled.
    let mut synced_seen: HashSet<PathBuf> = HashSet::new();
    let mut last_sync = Instant::now();
    const SYNC_INTERVAL: std::time::Duration = std::time::Duration::from_secs(300);

    // Havoc stage depth: how many executions each seed gets per pass. Each is a
    // fresh independent mutation stack from the pristine original, so this is
    // "run the seed many times", not "pile many mutations onto one buffer".
    const ENERGY: u32 = 256;

    while !stop.load(Ordering::Relaxed) {
        let mut new_finds: Vec<Vec<u8>> = Vec::new();
        for input in &node_pool {
            // Stop promptly on shutdown instead of finishing the whole pass.
            if stop.load(Ordering::Relaxed) {
                break;
            }

            // Havoc stage: spend ENERGY executions on this seed.
            for _ in 0..ENERGY {
                if stop.load(Ordering::Relaxed) {
                    break;
                }

                // Copy input
                let mut case = input.clone();

                // Mutate input
                random_havoc(&mut case);
                program_map.fill(0);

                let tmp = std::env::temp_dir().join(format!("input_{id}.tmp"));
                match std::fs::write(&tmp, &case) {
                    Ok(()) => {}
                    Err(e) => eprintln!("Error while writing the file to disk: {e}"),
                };

                let cmd_args = target_args.render(&tmp.to_string_lossy());

                // Redirect the child's stderr to a file (not a pipe). A pipe's
                // ~64KB buffer would fill and block a verbose child forever, since
                // we only drain stderr on a crash. A file never blocks the writer;
                // on a crash we read it back for the sanitizer report.
                let err_path = std::env::temp_dir().join(format!("stderr_{id}.log"));
                let err_file = match std::fs::File::create(&err_path) {
                    Ok(f) => Stdio::from(f),
                    Err(e) => {
                        eprintln!("Error creating stderr log: {e}");
                        Stdio::null()
                    }
                };

                let mut child = Command::new(&target)
                    .args(&cmd_args)
                    .env("ASTRA_SHM_ID", &shm_id)
                    .env("ASTRA_COV_MODE", &cov_mode)
                    .stdout(Stdio::null())
                    .stderr(err_file)
                    .spawn()
                    .expect("Failed to run the target");

                match child.wait_timeout(timeout_ms) {
                    // Catches crashes
                    Ok(Some(status)) => match status.signal() {
                        Some(sig @ (4 | 6 | 7 | 8 | 11)) => {
                            let report = std::fs::read_to_string(&err_path).unwrap_or_default();
                            let info = CrashInfo {
                                signal: sig,
                                report: &report,
                                argv: &cmd_args,
                            };
                            record_crash(&output_folder_path, &case, &info);
                            let _ = send_crash.send(true);
                        }
                        _ => {}
                    },

                    // Catches hangs
                    Ok(None) => {
                        match child.kill() {
                            Ok(()) => {}
                            Err(e) => eprintln!("Error while killing the child process: {e}"),
                        };

                        let _ = send_hang.send(true);
                        match child.wait() {
                            Ok((_status)) => {}
                            Err(e) => eprintln!("Error while waiting for child: {e}"),
                        };
                    }

                    Err(e) => eprintln!("Error while waiting for the child process: {e}"),
                };

                {
                    let flags = compare_maps(&node_map, program_map);
                    if flags.new_edge || flags.new_hitcount {
                        // Write file to disk
                        let name = format!("{:09}", file_index);
                        match std::fs::write(node_folder.join(name), &case) {
                            Ok(()) => {}
                            Err(e) => eprintln!("Error when joining folder: {e}"),
                        };

                        file_index = file_index + 1;

                        // Add input to new batch for next node pool
                        new_finds.push(case.clone());

                        // Unionize - merge map into node_map
                        union_maps(&mut node_map, program_map);

                        // Share new coverage map to main
                        match send_cov.send((id, node_map.clone())) {
                            Ok(()) => {}
                            Err(e) => eprintln!("Error while sending coverage: {e}"),
                        };

                        stats.tot_path.fetch_add(1, Ordering::Relaxed);
                    }
                }

                stats.tot_execution.fetch_add(1, Ordering::Relaxed);
            }
        }

        node_pool.extend(new_finds);

        // Periodically fold in finds from the other workers' node_* dirs.
        if sync && last_sync.elapsed() >= SYNC_INTERVAL {
            match sync_corpus(&output_folder_path, id, &mut synced_seen) {
                Ok(mut fresh) => node_pool.append(&mut fresh),
                Err(e) => eprintln!("worker {id}: corpus sync failed: {e}"),
            }
            last_sync = Instant::now();
        }
    }

    // Graceful shutdown: unmap and unlink this worker's shared memory.
    log_info!("Astra-worker", "The worker {id} is shutting down");
    clean_shared_memory(ptr, &shm_id);
}
