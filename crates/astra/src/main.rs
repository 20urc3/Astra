use astra_cli::*;
use astra_linker::*;
use astra_observer::{coverage_map, shm::*};

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
        .arg("")
        .spawn()
        .expect("failed to run child process");
    println!("Waiting for first child to end");
    child.wait().expect("child failed");


    // Initiate the edge map with the value of the first edge map
    let mut edge_map: &[u8] = unsafe { std::slice::from_raw_parts(ptr as *mut u8, MAP_SIZE) };

    // Initiate global map with the value of the first edge map 
    let mut global_map = edge_map.to_vec();

    println!("\nEdges found:");
    for (idx, &edge_count) in edge_map.iter().enumerate() {
        if edge_count != 0 {
            println!("edge_map[{}] = {}", idx, edge_count);
        }
    }
    
    println!("\nRunning the second process\n");
    unsafe {
        std::ptr::write_bytes(ptr, 0, MAP_SIZE);
    }
    // Spawn a new process with a file input that will triger a new coverage
    let mut second_child = Command::new("/home/s0urc3/Dev/Astra/a.out")
        .arg("")
        .spawn()
        .expect("failed to run child process");
    second_child.wait().expect("child failed");

    // Compare global map to new one
    if global_map == edge_map {
        println!("\nNo new coverage found");
    } else {

        for (idx, &edge_count) in edge_map.iter().enumerate() {
            if edge_count > 0 && global_map[idx] == 0 {
                println!("New edge found: edge_map[{}] = {}", idx, edge_count);
                global_map[idx] = edge_count;
            }
        }
    };

    println!("\nEdges found:");
    for (idx, &edge_count) in global_map.iter().enumerate() {
        if edge_count != 0 {
            println!("edge_map[{}] = {}", idx, edge_count);
        }
    }

    clean_shared_memory(ptr, shm_id.as_str());
}
