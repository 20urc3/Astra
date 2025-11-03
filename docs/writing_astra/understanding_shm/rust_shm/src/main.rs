use rustix::fs::{ftruncate, Mode};
use rustix::mm::{mmap, MapFlags, ProtFlags};
use rustix::{io, shm};
use std::mem::size_of;
use std::ptr::null_mut;
use std::process::Command;


#[repr(C)]
struct MyData {
    counter: i32,
    message: String,
}

fn main(){
    let name = "/my_shared_mem";
    let fd = shm::open(
        name,
        shm::OFlags::CREATE | shm::OFlags::EXCL | shm::OFlags::RDWR,
        Mode::RUSR | Mode::WUSR,
    ).unwrap();

    // Resize the shared memory object to the size of our data.
    ftruncate(&fd, size_of::<MyData>() as u64).unwrap();

    // Map the shared memory object into our address space.
    //
    // SAFETY: We're creating a new mapping that's independent of any existing
    // memory allocations. There are interesting things to say about *using*
    // `ptr`, but that's for another safety comment.
    let ptr = unsafe {
        mmap(
            null_mut(),
            size_of::<MyData>(),
            ProtFlags::READ | ProtFlags::WRITE,
            MapFlags::SHARED,
            &fd,
            0,
        ).unwrap()
    };

    // Using the shared memory
    let shared_data: &mut MyData = unsafe { &mut *(ptr as *mut MyData)} ;
    shared_data.message = String::from("Hello from shared memory!");

    // Read from shared memory
    let shared_data: &MyData = unsafe { &*(ptr as *const MyData) };
    println!("Reading message from memory: {}", shared_data.message);

    // Unmap memory
    unsafe { rustix::mm::munmap(ptr, size_of::<MyData>()) };

    // Remove the shared memory object name.
    shm::unlink(name).unwrap();

}
