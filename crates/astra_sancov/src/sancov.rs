use libc::{
    close, ftruncate, memcpy, mmap, shm_open, strncpy,
    MAP_SHARED, O_RDWR, O_CREAT, PROT_WRITE, S_IRUSR, S_IWUSR,
    c_char, c_void, off_t, size_t
};

use std::{env, ptr, str};
use std::process::Command;
use std::error::Error;
use std::ffi::CStr;
use std::io::Read;
use ctor::ctor;

const STORAGE_ID: *const c_char = c"ASTRA_SHM_ID".as_ptr() as *const c_char;
const STORAGE_SIZE : size_t = 128;
const null: *mut c_void = ptr::null_mut();
static mut edge_map: Vec<u8> = vec![];

#[ctor]
fn hello() {
    println!("this function should be ran at process startup!");
}

#[unsafe(no_mangle)]
pub extern "C" fn __sanitizer_cov_trace_pc_guard_init(start: *mut u32, stop: *mut u32) -> () {
    println!("START: {:?}", start);
    println!("STOP: {:?}", stop);
    let mut input_from_parent_process = [0u8; size_of::<i32>()];
    std::io::stdin()
        .read_exact(&mut input_from_parent_process)
        .expect("failed to read file descriptor");
    let fd = i32::from_ne_bytes(input_from_parent_process);
    println!("Attempting to mmap the shared memory file to the child process with fd: {}", fd);
    /*unsafe { 
        let addr = mmap(null, STORAGE_SIZE, PROT_WRITE, MAP_SHARED, fd, 0); 
        let _addr =  addr as  *const u8;
        let mut map: [u8] = unsafe { std::slice::from_raw_parts(_addr, 1)};
    }*/
}

#[unsafe(no_mangle)]
pub extern "C" fn __sanitizer_cov_trace_pc_guard(guard: u32) -> () {
    println!("GUARD: {:?}", guard);
    // Reading the fd from parent process
    //let mut input_from_parent_process = [0u8; size_of::<i32>()];
    //std::io::stdin()
    //    .read_exact(&mut input_from_parent_process)
    //    .expect("failed to read file descriptor");
    //let fd = i32::from_ne_bytes(input_from_parent_process);

    // Map the file to memory
    //unsafe { let addr = mmap(null, STORAGE_SIZE, PROT_WRITE, MAP_SHARED, fd, 0); }
    //unsafe { edge_map[guard as usize % edge_map.len()]; } // <- WRONG: Shared ref to a mutable static
}

