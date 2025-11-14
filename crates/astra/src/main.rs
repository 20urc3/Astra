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

    // Run workers
    running_workers(10,args.input_folder, args.target);

}
