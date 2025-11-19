pub mod worker;

use worker::*;

use astra_collector::collect_corpus;
use astra_observer::coverage::*;

const MAP_SIZE: usize = 262_144;

use std::{collections::VecDeque, path::PathBuf, thread};
use crossbeam::channel::unbounded;

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

    let mut global_map: Vec<u8> = vec![0u8; MAP_SIZE];
    let mut scheduler = Scheduler::new(corpus);

    loop {
        let input = scheduler.next_input();
        send_input.send(input.clone()).unwrap();

        while let Ok((_, input, new_map)) = recv_cov.try_recv() {
            let flags = compare_maps(global_map.as_slice(), new_map.as_slice());

            if flags.new_edge || flags.new_hit {

                println!("Global map is:");
                print_map(&global_map);
                copy_map(&new_map, &mut global_map);
                scheduler.add_favored(input);
            } else {
                scheduler.add_normal(input);
            }
        }
    }
}

struct Scheduler {
    normal: VecDeque<Vec<u8>>,
    favored: VecDeque<Vec<u8>>,
}

impl Scheduler {
    fn new(corpus: Vec<Vec<u8>>) -> Self {
        Self {
            normal: corpus.into_iter().collect(),
            favored: VecDeque::new(),
        }
    }

    fn next_input(&mut self) -> Vec<u8> {
        if let Some(f) = self.favored.pop_front() {
            f
        } else {
            let inp = self.normal.pop_front().unwrap();
            self.normal.push_back(inp.clone());
            inp
        }
    }

    fn add_favored(&mut self, input: Vec<u8>) {
        self.favored.push_back(input);
    }

    fn add_normal(&mut self, input: Vec<u8>) {
        self.normal.push_back(input);
    }
}
