//! Creates a shared memory file to save the code coverage from the child processes
//! 

use std::os::raw::c_void;
use rustix::fs::{ftruncate, Mode};
use rustix::mm::{mmap, MapFlags, ProtFlags};
use rustix::fd::OwnedFd;
use rustix::shm;
use std::ptr::null_mut;

const MAP_SIZE: usize = 262_144;

pub fn create_shared_memory(thr_id: u16) -> (OwnedFd, *mut c_void, String) {
    //println!("Initialization of shared memory.");
    let tid = thr_id.to_string();
    let mut shm_id = "/childprocess_".to_owned();
    shm_id.push_str(&tid);

    let fd = shm::open(
        &shm_id,
        shm::OFlags::CREATE | shm::OFlags::RDWR,
        Mode::RUSR | Mode::WUSR,
    ).unwrap();

    ftruncate(&fd, MAP_SIZE as u64).unwrap();

    let ptr = unsafe {
        mmap(
            null_mut(),
            MAP_SIZE,
            ProtFlags::READ | ProtFlags::WRITE,
            MapFlags::SHARED,
            &fd,
            0,
        ).unwrap()
    };

    println!("The memory was successfully initialized");

    (fd, ptr, shm_id.to_string())

}

pub fn clean_shared_memory(ptr: *mut c_void, shm_id: &str) {
    unsafe { rustix::mm::munmap(ptr, MAP_SIZE) };
    shm::unlink(shm_id).unwrap();

    println!("The memory was successfully cleaned");
}