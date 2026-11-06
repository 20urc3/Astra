pub mod files;
pub mod monitor;
pub mod scheduler;
pub mod stats;
pub mod worker;

use worker::*;

use crate::coverage::coverage::*;
use crate::coverage::shm::shm_name;
use crate::files::*;
use crate::log_info;
use crate::stats::*;
use crate::ui::cli::TargetArgs;

use crate::coverage::MAP_SIZE;

use chrono;
use colored_text::Colorize;
use flume::unbounded;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::{path::PathBuf, thread};

/// Setup sanitizer env arguments
/// With abort_on_error=1, ASan finishes every bug report by calling abort()
/// which raises SIGABRT (signal 6)
///
/// This means that no matter the bug type (overflow, UAF, even a SEGV it intercepted)
/// the process always dies with signal 6.
fn setup_sanitizer_args(leak: bool) {
    // Leak detection is off by default (short-lived fuzz targets "leak"
    // constantly on exit); `--leak` turns it back on.
    let detect_leaks = if leak { 1 } else { 0 };
    unsafe {
        std::env::set_var(
            "ASAN_OPTIONS",
            format!("abort_on_error=1:detect_leaks={detect_leaks}"),
        );
        std::env::set_var("UBSAN_OPTIONS", "halt_on_error=1:abort_on_error=1");
        std::env::set_var("TSAN_OPTIONS", "abort_on_error=1");
    }
}

/// Decrements the live-worker counter when a worker thread exits —
/// including on panic, since Drop runs during unwinding.
struct WorkerGuard(Arc<AtomicUsize>);
impl Drop for WorkerGuard {
    fn drop(&mut self) {
        self.0.fetch_sub(1, Ordering::Relaxed);
    }
}

/// Creates and run the worker pool
pub fn running_workers(
    num_thr: u16,
    output_folder: PathBuf,
    timeout: u64,
    target: PathBuf,
    target_args: TargetArgs,
    cov_mode: String,
    resume: bool,
    leak: bool,
    sync: bool,
) {
    // This allows to catchup ASan/UBsan etc exit code
    setup_sanitizer_args(leak);

    // Shared shutdown flag. Ctrl-C flips it; workers and the monitor loop
    // observe it and exit gracefully (each worker unmaps/unlinks its own
    // shared memory on the way out).
    let stop = Arc::new(AtomicBool::new(false));
    {
        let stop = stop.clone();
        let _ = ctrlc::set_handler(move || {
            stop.store(true, Ordering::Relaxed);
        });
    }

    // Number of worker threads still alive. Each worker increments on entry
    // and decrements on exit (via WorkerGuard, so a panic counts too).
    let live_workers = Arc::new(AtomicUsize::new(0));

    let (send_cov, recv_cov) = unbounded::<(u16, Vec<u8>)>();
    let (send_crash, recv_crash) = unbounded::<bool>();
    let (send_hang, recv_hang) = unbounded::<bool>();

    // Prepare map and statistics
    let mut global_map = vec![0u8; MAP_SIZE];
    let fuzz_stats = Arc::new(FuzzingStats::new());
    let start_time_instant = std::time::Instant::now();
    let mut last_print_time = std::time::Instant::now();
    let mut last_time_new_path = std::time::Instant::now();
    let mut speed_window = SpeedWindow::new();

    // Load the shared seed corpus once; every worker starts from a clone of
    // it, so we don't read the same seeds/ directory N times.
    let shared_seeds = load_corpus(&output_folder.join("seeds")).unwrap_or_else(|error| {
        eprintln!("failed to load seeds: {error}");
        Vec::new()
    });

    // Spawn the threads
    let mut worker_handles = Vec::new();
    for id in 0..num_thr {
        // Prepare this worker's node directory and initial pool on disk
        // (shared seeds + its own prior finds when resuming).
        let (node_folder, node_pool, file_index) =
            match init_worker_corpus(&output_folder, id, resume, &shared_seeds) {
                Ok(v) => v,
                Err(error) => {
                    eprintln!("worker {id}: failed to prepare corpus: {error}");
                    continue;
                }
            };

        let output_folder = output_folder.clone();
        let send_cov = send_cov.clone();
        let send_crash = send_crash.clone();
        let send_hang = send_hang.clone();
        let stats = fuzz_stats.clone();
        let target = target.clone();
        let target_args = target_args.clone();
        let cov_mode = cov_mode.clone();
        let stop = stop.clone();
        let live_workers = live_workers.clone();

        worker_handles.push(thread::spawn(move || {
            live_workers.fetch_add(1, Ordering::Relaxed);
            let _guard = WorkerGuard(live_workers.clone());
            worker(
                id,
                output_folder,
                node_folder,
                node_pool,
                file_index,
                target,
                timeout,
                target_args,
                stats,
                send_cov,
                send_crash,
                send_hang,
                cov_mode,
                stop,
                sync,
            )
        }));
    }

    // Workers that actually launched (corpus-prep failures `continue` above).
    let spawned = worker_handles.len();

    while !stop.load(Ordering::Relaxed) {
        let elapsed_secs = start_time_instant.elapsed().as_secs();
        fuzz_stats.run_time.store(elapsed_secs, Ordering::Relaxed);

        // Checking for findings
        while let Ok((_, child_map)) = recv_cov.try_recv() {
            union_maps(&mut global_map, &child_map);

            last_time_new_path = std::time::Instant::now();

            fuzz_stats
                .raw_edges
                .store(count_raw_edges(&global_map), Ordering::Relaxed);
            fuzz_stats
                .raw_hits
                .store(total_raw_hits(&global_map), Ordering::Relaxed);
        }

        // Checking for crashes
        while let Ok(_) = recv_crash.try_recv() {
            fuzz_stats.tot_crash.fetch_add(1, Ordering::Relaxed);
        }

        // Checking for hangs
        while let Ok(_) = recv_hang.try_recv() {
            fuzz_stats.tot_tmout.fetch_add(1, Ordering::Relaxed);
        }

        let t_since_last = last_time_new_path.elapsed().as_secs();
        fuzz_stats
            .t_since_last_path
            .store(t_since_last, Ordering::Relaxed);

        if last_print_time.elapsed() >= std::time::Duration::new(1, 0) {
            let live = live_workers.load(Ordering::Relaxed);
            if live < spawned {
                log_info!("Astra-fuzz", "WARNING: {} worker(s) died", spawned - live);
                if live == 0 {
                    log_info!("Astra-fuzz", "all workers died — shutting down");
                    stop.store(true, Ordering::Relaxed);
                }
            }

            let runtime = fuzz_stats
                .run_time
                .load(std::sync::atomic::Ordering::Relaxed);
            let t_since_last_path = fuzz_stats
                .t_since_last_path
                .load(std::sync::atomic::Ordering::Relaxed);
            let tot_path = fuzz_stats
                .tot_path
                .load(std::sync::atomic::Ordering::Relaxed);
            let raw_edges = fuzz_stats
                .raw_edges
                .load(std::sync::atomic::Ordering::Relaxed);
            let raw_hits = fuzz_stats
                .raw_hits
                .load(std::sync::atomic::Ordering::Relaxed);
            let tot_crash = fuzz_stats
                .tot_crash
                .load(std::sync::atomic::Ordering::Relaxed);
            let tot_tmout = fuzz_stats
                .tot_tmout
                .load(std::sync::atomic::Ordering::Relaxed);
            let tot_execution = fuzz_stats.tot_execution.load(Ordering::Relaxed);
            // Instantaneous speed over the gap since the last print.
            let exec_speed = speed_window.tick(tot_execution);
            fuzz_stats.exec_speed.store(exec_speed, Ordering::Relaxed);

            let elapsed = Runtime::from_secs(runtime).display();

            log_info!(
                "Astra-worker",
                "runtime: {elapsed} | live: {live}/{spawned} | time since last find: {} | total findings: {} | tot hit/edges: {}/{} | crash/timeout: {}/{}  | total exec: {} | exec/sec: {}",
                t_since_last_path,
                tot_path,
                raw_edges,
                raw_hits,
                tot_crash,
                tot_tmout,
                tot_execution,
                exec_speed
            );

            last_print_time = std::time::Instant::now();
        }
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }

    // Shutdown requested: wait for workers to finish their current execution
    // and clean up their own shared memory.
    log_info!("Astra-fuzz", "Shutting down, waiting for workers...");
    for handle in worker_handles {
        let _ = handle.join();
    }

    // Backstop: unlink any segment a panicked worker may have left behind.
    for id in 0..num_thr {
        let _ = rustix::shm::unlink(&shm_name(id));
    }

    log_info!("Astra-fuzz", "Shutdown complete");
}
