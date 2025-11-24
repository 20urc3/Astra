mod log_macro;

use std::time::Instant;

pub struct FuzzingStats {
    pub start_time: Instant,
    pub run_time: f64,
    pub tot_execution: f64,
    pub t_since_last_path: f64,
    pub t_since_last_crash: u32,
    pub t_since_last_timeout: u32,
    pub tot_crash: u32,
    pub tot_tmout: u32,
    pub tot_path: u32,
    pub exec_speed: f64,
    pub raw_edges: usize,
    pub raw_hits: usize,
}

impl FuzzingStats {
    pub fn new() -> FuzzingStats {
        let start_time = Instant::now();
        let run_time: f64 = 0.0;
        let tot_execution: f64 = 0.0;
        let t_since_last_path: f64 = 0.0;
        let t_since_last_crash: u32 = 0;
        let t_since_last_timeout: u32 = 0;
        let tot_crash: u32 = 0;
        let tot_tmout: u32 = 0;
        let tot_path: u32 = 0;
        let exec_speed: f64 = 0.0;
        let raw_edges = 0;
        let raw_hits = 0;

        Self {
            start_time,
            run_time,
            tot_execution,
            t_since_last_path,
            t_since_last_crash,
            t_since_last_timeout,
            tot_crash,
            tot_tmout,
            tot_path,
            exec_speed,
            raw_edges,
            raw_hits,
        }
    }
}

