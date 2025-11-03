//! Creates a shared memory file to save the code coverage from the child processes

use libc::{
    close, ftruncate, memcpy, mmap, shm_open, strncpy,
    MAP_SHARED, O_RDWR, O_CREAT, PROT_WRITE, S_IRUSR, S_IWUSR,
    c_char, c_void, off_t, size_t
};

use std::{env, ptr, str};
use std::process::Command;
use std::error::Error;
use std::ffi::CStr;

pub fn create_shared_memory() -> i32 {
        let (fd, addr) = unsafe {
            const STORAGE_ID: *const c_char = c"ASTRA_SHM_ID".as_ptr() as *const c_char;
            const STORAGE_SIZE : size_t = 128;
            let null = ptr::null_mut();
            let fd   = shm_open(STORAGE_ID, O_RDWR | O_CREAT, ((S_IRUSR | S_IWUSR) as size_t).try_into().unwrap());
            let _res = ftruncate(fd, STORAGE_SIZE as off_t);
            let addr = mmap(null, STORAGE_SIZE, PROT_WRITE, MAP_SHARED, fd, 0);
         
            (fd, addr)
        };
    fd
}