mod log_macro;

use std::sync::atomic::{AtomicU32, AtomicU64, AtomicUsize};

pub struct FuzzingStats {
    pub run_time: AtomicU64,     
    pub tot_execution: AtomicU64,   
    pub t_since_last_path: AtomicU64, 
    pub t_since_last_crash: AtomicU32,
    pub t_since_last_timeout: AtomicU32,
    pub tot_crash: AtomicU32,
    pub tot_tmout: AtomicU32,
    pub tot_path: AtomicU32,
    pub exec_speed: AtomicU64,    
    pub raw_edges: AtomicUsize,
    pub raw_hits: AtomicUsize,
}

impl FuzzingStats {
    pub fn new() -> FuzzingStats {
        Self {
            run_time: AtomicU64::new(0),
            tot_execution: AtomicU64::new(0),
            t_since_last_path: AtomicU64::new(0),
            t_since_last_crash: AtomicU32::new(0),
            t_since_last_timeout: AtomicU32::new(0),
            tot_crash: AtomicU32::new(0),
            tot_tmout: AtomicU32::new(0),
            tot_path: AtomicU32::new(0),
            exec_speed: AtomicU64::new(0),
            raw_edges: AtomicUsize::new(0),
            raw_hits: AtomicUsize::new(0),
        }
    }
}