use astra_cli::*;
use astra_linker::*;
use astra_observer::shm::*;

use clap::Parser;
use std::os::fd::AsRawFd;
use std::process::Command;

const MAP_SIZE: usize = 262_144;

fn main() {
    let args = Args::parse();
    println!("You passed the program to test: {:?}", args.program);
    println!("Attempting to link the target program with astra_sancov library");

    linking_target_to_sancov(args.program);

    let (fd, ptr, shm_id) = create_shared_memory();

    let mut child = Command::new("/home/s0urc3/Dev/Astra/a.out")
        .arg("testfile")
        .spawn()
        .expect("failed to run child process");

    child.wait().expect("child failed");



    let edge_map = unsafe { std::slice::from_raw_parts(ptr as *const u8, MAP_SIZE) };

    println!("\nEdges found:");
    for (i, &v) in edge_map.iter().enumerate() {
        if v != 0 {
            println!("edge_map[{}] = {}", i, v);
        }
    }

    clean_shared_memory(ptr, shm_id.as_str());
}
