use astra_cli::*;
use astra_linker::*;
use astra_observer::{coverage::*, shm::*};
use astra_collector::*;
use astra_scheduler::{mutation_queue::*, testcase_queue::*};

use astra_worker::worker::worker;
use astra_worker::*;
use clap::Parser;
use std::os::fd::AsRawFd;
use std::process::Command;
use std::sync::Arc;

const MAP_SIZE: usize = 262_144;

fn main() {
    // Parsing arguments
    let args = Args::parse();
    println!("You passed the program to link: {:?}", args.program);
    println!("You passed the program to test: {:?}", args.target);

    // Linking custom sancov to target program
    println!("Attempting to link the target program with astra_sancov library");
    linking_target_to_sancov(&args.program);

    // Initialize the shared objects to pass to threads
    initiate_shared_objects(args.input_folder, &args.program);

    let target_copy = Arc::new(args.target);
    //running_workers(1, args.program);
    worker(1, target_copy);

    // Create the shared memory file
    /*let (fd, ptr, shm_id) = create_shared_memory();
    // Map the local edge_map with the shared memory edge_map
    let edge_map: &[u8] = unsafe { std::slice::from_raw_parts(ptr as *mut u8, MAP_SIZE) };

    // Running the first process
    println!("\nRunning the first process:");
    let mut child = Command::new("./a.out")
        .arg("")
        .spawn()
        .expect("failed to run child process");
    child.wait().expect("child failed");
    
    // Initiate global map with the value of the first edge map 
    let mut global_map = populate_global_map(edge_map);

    // print edge found in the first process run
    println!("\nThe edge map produced by this process is:");
    print_edge_found(edge_map);

    // Cleaning the shared map
    unsafe { std::ptr::write_bytes(ptr, 0, MAP_SIZE); }

    // Spawning a second process with a file input that will triger a new coverage
    println!("\nRunning the second process:");
    let mut second_child = Command::new("./a.out")
        .arg("./docs/writing_astra/coverage_example/testfile_1")
        .spawn()
        .expect("failed to run child process");
    second_child.wait().expect("child failed");

    println!("\nThe edge map produced by this process is:");
    print_edge_found(edge_map);


    // Compare global map to new one
    let flags = compare_global_to_edge(&edge_map, &mut global_map);
    if flags.new_edge { println!("[NEW_EDGE] - The edge_map contains new edge(s)."); }
    if flags.new_hit  { println!("[NEW_HIT] - The hit-count of some edge has increased."); }

    // Cleaning the shared map
    unsafe { std::ptr::write_bytes(ptr, 0, MAP_SIZE); }

    // Spawning a third process with a file input that will triger a new coverage
    println!("\nRunning the third process:");
    let mut second_child = Command::new("./a.out")
        .arg("./docs/writing_astra/coverage_example/testfile_2")
        .spawn()
        .expect("failed to run child process");
    second_child.wait().expect("child failed");

    println!("\nThe edge map produced by this process is:");
    print_edge_found(edge_map);


    // Compare global map to new one
    let flags = compare_global_to_edge(&edge_map, &mut global_map);
    if flags.new_edge { println!("[NEW_EDGE] - The edge_map contains new edge(s)."); }
    if flags.new_hit  { println!("[NEW_HIT] - The hit-count of some edge has increased."); }

    // Print stats
    println!("\nThe global map coverage is:");
    print_edge_found(&global_map);

    // Clean shared memory
    clean_shared_memory(ptr, shm_id.as_str());*/
}
