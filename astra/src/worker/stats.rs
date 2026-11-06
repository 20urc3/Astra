use std::sync::atomic::{AtomicU32, AtomicU64, AtomicUsize};
use std::time::Instant;

pub struct FuzzingStats {
    pub run_time: AtomicU64,
    pub tot_execution: AtomicU64,
    pub t_since_last_path: AtomicU64,
    pub t_since_last_crash: AtomicU32,
    pub t_since_last_timeout: AtomicU32,
    pub tot_crash: AtomicU64,
    pub tot_tmout: AtomicU64,
    pub tot_path: AtomicU64,
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
            tot_crash: AtomicU64::new(0),
            tot_tmout: AtomicU64::new(0),
            tot_path: AtomicU64::new(0),
            exec_speed: AtomicU64::new(0),
            raw_edges: AtomicUsize::new(0),
            raw_hits: AtomicUsize::new(0),
        }
    }
}

/// Rolling-window exec-speed tracker. Call `tick` once per print cycle: it
/// returns execs/sec over the gap since the previous call, not the lifetime
/// average, so it reacts immediately when cores stall or die.
pub struct SpeedWindow {
    last_exec: u64,
    last_tick: Instant,
}

impl SpeedWindow {
    pub fn new() -> Self {
        Self {
            last_exec: 0,
            last_tick: Instant::now(),
        }
    }

    /// Given the current cumulative execution count, return the instantaneous
    /// speed over the elapsed window and reset the window.
    pub fn tick(&mut self, tot_exec: u64) -> u64 {
        let dt = self.last_tick.elapsed().as_secs_f64();
        let speed = if dt > 0.0 {
            ((tot_exec.saturating_sub(self.last_exec)) as f64 / dt) as u64
        } else {
            0
        };
        self.last_exec = tot_exec;
        self.last_tick = Instant::now();
        speed
    }
}

/// Display nicely the runtime
///  After 60 seconds run display time in minute::seconds
/// After 60 minutes run display time in hour::minutes
/// After 24 hours run display time in day::hours
pub struct Runtime {
    days: u64,
    hours: u64,
    minutes: u64,
    seconds: u64,
}

impl Runtime {
    pub fn from_secs(total: u64) -> Self {
        Runtime {
            days: total / 86_400,
            hours: (total % 86_400) / 3_600,
            minutes: (total % 3_600) / 60,
            seconds: total % 60,
        }
    }

    /// Display the two most significant non-zero units.
    pub fn display(&self) -> String {
        if self.days > 0 {
            format!("{}days{:02}h", self.days, self.hours)
        } else if self.hours > 0 {
            format!("{}hours{:02}m", self.hours, self.minutes)
        } else if self.minutes > 0 {
            format!("{}min{:02}s", self.minutes, self.seconds)
        } else {
            format!("{}sec", self.seconds)
        }
    }
}
