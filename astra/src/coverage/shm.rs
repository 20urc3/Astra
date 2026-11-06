//! Creates a shared memory file to save the code coverage from the child processes
//!

use rustix::fd::OwnedFd;
use rustix::fs::{Mode, ftruncate};
use rustix::io::Errno;
use rustix::mm::{MapFlags, ProtFlags, mmap};
use rustix::shm;
use std::os::raw::c_void;
use std::ptr::null_mut;

use crate::coverage::MAP_SIZE;

/// Shared-memory name prefix. Single source of truth so the segment
/// creator and the shutdown cleanup can't drift apart.
pub const SHM_PREFIX: &str = "/childprocess_";

/// Deterministic shared-memory name for a given worker id.
pub fn shm_name(thr_id: u16) -> String {
    format!("{SHM_PREFIX}{thr_id}")
}

pub fn create_shared_memory(thr_id: u16) -> Result<(OwnedFd, *mut c_void, String), Errno> {
    let shm_id = shm_name(thr_id);

    // Cleanup/defensive cleanup
    let _ = shm::unlink(&shm_id);

    let fd = match shm::open(
        &shm_id,
        shm::OFlags::CREATE | shm::OFlags::RDWR,
        Mode::RUSR | Mode::WUSR,
    ) {
        Ok(fd) => fd,
        Err(e) => {
            eprintln!("failed to open shared memory '{shm_id}': {e}");
            return Err(e); //
        }
    };

    if let Err(e) = ftruncate(&fd, MAP_SIZE as u64) {
        eprintln!("failed to resize shared memory to {MAP_SIZE} bytes: {e}");
        return Err(e);
    }

    let ptr = match unsafe {
        mmap(
            null_mut(),
            MAP_SIZE,
            ProtFlags::READ | ProtFlags::WRITE,
            MapFlags::SHARED,
            &fd,
            0,
        )
    } {
        Ok(ptr) => ptr,
        Err(e) => {
            eprintln!("failed to mmap shared memory '{shm_id}': {e}");
            return Err(e);
        }
    };

    // success: wrap the tuple in Ok
    Ok((fd, ptr, shm_id))
}

pub fn clean_shared_memory(ptr: *mut c_void, shm_id: &str) {
    // Best-effort: the segment may already be gone (e.g. a defensive
    // unlink from a concurrent restart), so don't panic on failure.
    let _ = unsafe { rustix::mm::munmap(ptr, MAP_SIZE) };
    let _ = shm::unlink(shm_id);
}
