use libc::c_void;
use std::ptr::null_mut;
use rustix::{
    fs::{Mode, ftruncate},
    mm::{MapFlags, ProtFlags, mmap},
    shm,
};

// Shared memory pointer
static mut SHM_PTR: *mut c_void = std::ptr::null_mut();

// Constant map size for the edge map
const MAP_SIZE: usize = 262_144;

/// This initiliziation function is *supposedly* ran only once per DSO
/// It opens the shared memory file only if it exist
/// Then map it to a global variable SHM_PTR used to write the coverage
#[unsafe(no_mangle)]
pub extern "C" fn __sanitizer_cov_trace_pc_guard_init(mut start: *mut u32, stop: *mut u32) -> () {
    

    let thr_id = std::env::var("ASTRA_THR_ID").expect("Missing ASTRA_THR_ID");
    let shm_id = format!("/childprocess_{}", thr_id);

    let fd = shm::open(&shm_id, shm::OFlags::RDWR, Mode::RUSR | Mode::WUSR).unwrap();

    let ptr: *mut c_void = unsafe {
        mmap(
            null_mut(),
            MAP_SIZE,
            ProtFlags::READ | ProtFlags::WRITE,
            MapFlags::SHARED,
            &fd,
            0,
        )
        .unwrap()
    };

    ftruncate(&fd, MAP_SIZE as u64).unwrap();

    // Assigning the mmaped `ptr` to SHM_PTR global variable
    unsafe { SHM_PTR = ptr; }

    // We always start at 1
    static mut N: u32 = 1; 
    // Assert that the edge map isn't bigger than the shared memory.
    if (unsafe { stop.offset_from(start)}) > MAP_SIZE.try_into().unwrap() { return; };
    unsafe { if start == stop || *start != 0 { return; } };
    while start < stop {
        unsafe {
            *start = N;
            N += 1;
            start = start.add(1);
        }
    }

}

/// This function is called every time an edge is seen
/// It tracks edge coverage by assigninga unique ID per edge 
/// and keeps count of the number of time an edge is seen.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __sanitizer_cov_trace_pc_guard(guard: *mut u32) -> () {
    let edge_map: &mut [u8] = unsafe { std::slice::from_raw_parts_mut(SHM_PTR as *mut u8, MAP_SIZE) };
    let idx = unsafe { (*guard as usize) % MAP_SIZE };
    edge_map[idx] = edge_map[idx].wrapping_add(1);
    //println!("Marked edge: {}", *guard);
}
