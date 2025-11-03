//! Creates a shared memory file to save the code coverage from the child processes
//! 
use std::ffi::CString;
use std::os::raw::c_void;
use rustix::fs::{ftruncate, Mode};
use rustix::mm::{mmap, MapFlags, ProtFlags};
use rustix::fd::OwnedFd;
use rustix::{io, shm};
use std::mem::size_of;
use std::ptr::null_mut;
use std::process::Command;

pub fn create_shared_memory() -> (OwnedFd, *mut c_void, String) {
    println!("Initialization of shared memory.");
    let shm_id = "ASTRA_SHM_IDBB";
    let fd = shm::open(
        shm_id,
        shm::OFlags::CREATE | shm::OFlags::EXCL | shm::OFlags::RDWR,
        Mode::RUSR | Mode::WUSR,
    ).unwrap();

    ftruncate(&fd, size_of::<u8>() as u64).unwrap();

    let ptr = unsafe {
        mmap(
            null_mut(),
            size_of::<u8>(),
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
    unsafe { rustix::mm::munmap(ptr, size_of::<u8>()) };
    shm::unlink(shm_id).unwrap();

    println!("The memory was successfully cleaned");
}