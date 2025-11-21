pub mod worker;

use worker::*;

use astra_collector::collect_corpus;
use astra_observer::coverage::*;
use astra_tui::*;

const MAP_SIZE: usize = 262_144;

use std::{fmt::Arguments, path::PathBuf, thread};
use crossbeam::channel::unbounded;

/// Creates and run the worker pool

pub fn running_workers(num_thr: u16, input_dir: PathBuf, target: PathBuf, arguments: Vec<String>) {
    let (send_input, recv_input) = unbounded::<Vec<u8>>();
    let (send_cov, recv_cov) = unbounded::<(u16, Vec<u8>, Vec<u8>)>();
    let (send_finding, recv_finding) = unbounded::<bool>();

    for id in 0..num_thr {
        let recv_input = recv_input.clone();
        let send_cov = send_cov.clone();
        let send_finding = send_finding.clone();
        let target = target.clone();
        let arguments = arguments.clone();
        thread::spawn(move || worker(id, target, arguments, recv_input, send_cov, send_finding));
    }

    let mut corpus = collect_corpus(&input_dir);
    assert!(!corpus.is_empty());

    let mut global_map = vec![0u8; MAP_SIZE];
    let mut favored_inputs: Vec<Vec<u8>> = Vec::new();

    let mut fuzz_stats = FuzzingStats::new();
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

        while let Ok((_, input, new_map)) = recv_cov.try_recv() {
            let flags = compare_maps(&global_map, &new_map);
            if flags.new_edge || flags.new_hit {
                println!("Global map is:");
                print_map(&global_map);
                copy_map(&new_map, &mut global_map);
                favored_inputs.push(input);
                fuzz_stats.tot_path += 1;
                last_time_new_path = std::time::Instant::now();

            } else {
                corpus.push(input);
                
            }
            fuzz_stats.tot_execution += 1.0;
            fuzz_stats.run_time = fuzz_stats.start_time.elapsed().as_secs_f64();
            fuzz_stats.exec_speed = fuzz_stats.tot_execution / fuzz_stats.run_time;
            fuzz_stats.t_since_last_path = last_time_new_path.elapsed().as_secs_f64();

        }
        
        while let Ok(finding) = recv_finding.try_recv() {
            fuzz_stats.tot_crash += 1;
        
        }

        
        if last_print_time.elapsed() >= std::time::Duration::new(1, 0) {
            println!(
                "runtime: {:.0} secs | time since last find: {:.0} | total findings: {} | crash/timeout: {:?}/{:?}  | total exec: {:?} | exec/sec: {:.2}",
                fuzz_stats.run_time,
                fuzz_stats.t_since_last_path,
                fuzz_stats.tot_path,
                fuzz_stats.tot_crash,
                fuzz_stats.tot_tmout,
                fuzz_stats.tot_execution,
                fuzz_stats.exec_speed
            );
            last_print_time = std::time::Instant::now();
        }
    }
}
