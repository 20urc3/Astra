use std::sync::atomic::{AtomicPtr, Ordering};
use std::ptr;
use ctor::ctor;
use std::{env, fs::File, os::fd::{FromRawFd, AsRawFd}};
use libc::{mmap, MAP_SHARED, PROT_READ, PROT_WRITE, MAP_FAILED};


#[unsafe(no_mangle)]
pub extern "C" fn __sanitizer_cov_trace_pc_guard_init(start: *mut u32, stop: *mut u32) -> () {
    println!("START: {:?}", start);
    println!("STOP: {:?}", stop);
}

#[unsafe(no_mangle)]
pub extern "C" fn __sanitizer_cov_trace_pc_guard(guard: u32) -> () {
    println!("GUARD: {:?}", guard);
}
