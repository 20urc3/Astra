use astra_observer::shm::*;
use ctor::ctor;
use std::ptr::null_mut;
use libc::c_void;
use rustix::{
    shm, 
    fs::{
        Mode, ftruncate,
    }, 
    fd::{
        OwnedFd,
    },
    mm::{
        mmap, MapFlags, ProtFlags
    }
};

static mut SHM_PTR: *mut c_void = std::ptr::null_mut();
const MAP_SIZE: usize = 262_144;

#[ctor]
fn init_shm (){
    println!("Initialization of shared memory.");
    let shm_id = "/astra_shm";
    let fd = shm::open(
        shm_id,
        shm::OFlags::RDWR,
        Mode::RUSR | Mode::WUSR,
    ).unwrap();

    let ptr: *mut c_void = unsafe {
        mmap(
            null_mut(),
            MAP_SIZE,
            ProtFlags::READ | ProtFlags::WRITE,
            MapFlags::SHARED,
            &fd,
            0,
        ).unwrap()
    };

    unsafe { SHM_PTR = ptr; }
}


#[unsafe(no_mangle)]
pub extern "C" fn __sanitizer_cov_trace_pc_guard_init(start: *mut u32, stop: *mut u32) -> () {
    println!("START: {:?}", start);
    println!("STOP: {:?}", stop);
}

#[unsafe(no_mangle)]
pub extern "C" fn __sanitizer_cov_trace_pc_guard(guard: u32) -> () {
    let edge_map: &mut [u8] = unsafe { std::slice::from_raw_parts_mut(SHM_PTR as *mut u8, MAP_SIZE) };
    let idx = (guard as usize) % MAP_SIZE;
    edge_map[idx] = edge_map[idx].wrapping_add(1);
    println!("Marked edge: {}", guard);
}
