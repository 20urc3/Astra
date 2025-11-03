use astra_cli::*;
use astra_linker::*;
use astra_observer::shm::*;

use clap::Parser;
use std::io::{Read, Write};

fn main() {
    let args = Args::parse();
    println!("You passed the program to test: {:?}", args.program);
    println!("Attempting to link the target program with astra_sancov library");
    
    linking_target_to_sancov(args.program);

    let fd = create_shared_memory();
    let mut child = std::process::Command::new("/home/s0urc3/Dev/Astra/a.out")
        .stdin(std::process::Stdio::piped())
        .spawn()
        .expect("failed to run child process");

    let mut child_input = child.stdin.take().unwrap();
    let fd_b = fd.to_ne_bytes();
    let _ = child_input.write_all(&fd_b);


}
