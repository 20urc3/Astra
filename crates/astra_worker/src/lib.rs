pub mod worker;

use worker::*;

use astra_collector::collect_corpus;
use astra_observer::coverage::*;
use astra_tui::*;

const MAP_SIZE: usize = 262_144;

use std::{path::PathBuf, thread};
use flume::unbounded;
use std::sync::atomic::Ordering;
use colored_text::Colorize;
use chrono;

/// Creates and run the worker pool
pub fn running_workers(num_thr: u16, input_dir: PathBuf, timeout: u64, target: PathBuf, arguments: Vec<String>) {
    let (send_input, recv_input) = unbounded::<Vec<u8>>();
    let (send_cov, recv_cov) = unbounded::<(u16, Vec<u8>, Vec<u8>)>();
    let (send_crash, recv_crash) = unbounded::<bool>();
    let (send_hang, recv_hang) = unbounded::<bool>();

    let mut worker_handles = Vec::new();
    for id in 0..num_thr {
        let recv_input = recv_input.clone();
        let send_cov = send_cov.clone();
        let send_crash = send_crash.clone();
        let send_hang = send_hang.clone();
        let target = target.clone();
        let arguments = arguments.clone();

        worker_handles.push(thread::spawn(move || worker(id, target, timeout, arguments, recv_input, send_cov, send_crash, send_hang)));
    }

    let mut corpus = collect_corpus(&input_dir);
    assert!(!corpus.is_empty());

    let mut global_map = vec![0u8; MAP_SIZE];
    let mut favored_inputs: Vec<Vec<u8>> = Vec::new();

    let fuzz_stats = FuzzingStats::new();
    let start_time_instant = std::time::Instant::now();
    let mut last_print_time = std::time::Instant::now();
    let mut last_time_new_path = std::time::Instant::now();

    loop {
        if let Some(input) = favored_inputs.pop() {
            send_input.send(input.clone()).unwrap();
        } else {
            let mut corpus_clone = corpus.clone();
            let input = corpus_clone.pop().unwrap();
            send_input.send(input).unwrap();
        }

        while let Ok((_, input, child_map)) = recv_cov.try_recv() {
            
            let flags = compare_maps(&mut global_map, &child_map);
            
            if flags.new_edge || flags.new_hit {
                favored_inputs.push(input);
                fuzz_stats.tot_path.fetch_add(1, Ordering::Relaxed);
                last_time_new_path = std::time::Instant::now();

            } else {
                corpus.push(input);
            }
            
            copy_map(&mut global_map, &child_map);
            fuzz_stats.tot_execution.fetch_add(1, Ordering::Relaxed);

            let elapsed_secs = start_time_instant.elapsed().as_secs();
            fuzz_stats.run_time.store(elapsed_secs, Ordering::Relaxed);

            let tot_exec = fuzz_stats.tot_execution.load(Ordering::Relaxed);
            let exec_speed = if elapsed_secs > 0 { tot_exec / elapsed_secs } else { 0 };
            fuzz_stats.exec_speed.store(exec_speed, Ordering::Relaxed);

            let t_since_last = last_time_new_path.elapsed().as_secs();
            fuzz_stats.t_since_last_path.store(t_since_last, Ordering::Relaxed);

            fuzz_stats.raw_edges.store(count_raw_edges(&global_map), Ordering::Relaxed);
            fuzz_stats.raw_hits.store(total_raw_hits(&global_map), Ordering::Relaxed);

        }
        
        while let Ok(_) = recv_crash.try_recv() {
            fuzz_stats.tot_crash.fetch_add(1, Ordering::Relaxed);
        
        }

        while let Ok(_) = recv_hang.try_recv() {
            fuzz_stats.tot_tmout.fetch_add(1, Ordering::Relaxed);
        }

        
        if last_print_time.elapsed() >= std::time::Duration::new(1, 0) {
            let runtime = fuzz_stats.run_time.load(std::sync::atomic::Ordering::Relaxed);
            let t_since_last_path = fuzz_stats.t_since_last_path.load(std::sync::atomic::Ordering::Relaxed);
            let tot_path = fuzz_stats.tot_path.load(std::sync::atomic::Ordering::Relaxed);
            let raw_edges = fuzz_stats.raw_edges.load(std::sync::atomic::Ordering::Relaxed);
            let raw_hits = fuzz_stats.raw_hits.load(std::sync::atomic::Ordering::Relaxed);
            let tot_crash = fuzz_stats.tot_crash.load(std::sync::atomic::Ordering::Relaxed);
            let tot_tmout = fuzz_stats.tot_tmout.load(std::sync::atomic::Ordering::Relaxed);
            let tot_execution = fuzz_stats.tot_execution.load(std::sync::atomic::Ordering::Relaxed);
            let exec_speed = fuzz_stats.exec_speed.load(std::sync::atomic::Ordering::Relaxed);
            
            log_info!(
                "Astra-worker",
                "runtime: {} secs | time since last find: {} | total findings: {} | tot edges/hit: {}/{} | crash/timeout: {}/{}  | total exec: {} | exec/sec: {}",
                runtime,
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
    }
}
