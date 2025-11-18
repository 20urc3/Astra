pub mod worker;

use worker::*;

use astra_collector::collect_corpus;
use astra_observer::coverage::*;

use crossbeam::channel::unbounded;
use std::{
    thread, 
    path::PathBuf,
};

const MAP_SIZE: usize = 262_144;

/// Creates and run the worker pool
pub fn running_workers(num_thr: u16, input_dir: PathBuf, target: PathBuf) {

    let (send_input, recv_input) = unbounded::<Vec<u8>>();
    let (send_cov, recv_cov) = unbounded::<(u16, Vec<u8>, Vec<u8>)>();


    for id in 0..num_thr {
        let recv_input = recv_input.clone();
        let send_cov = send_cov.clone();
        let target = target.clone();
        thread::spawn(move || worker(id, target, recv_input, send_cov));
    }


    let corpus = collect_corpus(&input_dir);
    assert!(!corpus.is_empty());

    let mut global_map = vec![0u8; MAP_SIZE];
    let mut favored_inputs: Vec<Vec<u8>> = Vec::new();
    let mut corpus_iter = corpus.iter().cycle();

    loop {

        if let Some(input) = favored_inputs.pop() {
            send_input.send(input.clone()).unwrap();
        } else {
            let input = corpus_iter.next().unwrap();
            send_input.send(input.clone()).unwrap();
        }

        while let Ok((_, input, new_map)) = recv_cov.try_recv() {
            let flags = compare_maps(&global_map, &new_map);

            if flags.new_edge || flags.new_hit {
                println!("Global map is:");
                print_map(&global_map);
                copy_map(&new_map, &mut global_map);
                favored_inputs.push(input);
            }
        }
    }
}