use astra_cli::*;
use astra_linker::*;
use astra_observer::shm::*;

use clap::Parser;
use std::os::fd::AsRawFd;
use std::process::Command;

fn main() {
    let args = Args::parse();
    println!("You passed the program to test: {:?}", args.program);
    println!("Attempting to link the target program with astra_sancov library");

    linking_target_to_sancov(args.program);

    let (fd, ptr, shm_id) = create_shared_memory();
    let fdsc = fd.as_raw_fd();

    let mut child = Command::new("/home/s0urc3/Dev/Astra/a.out")
        .env("SHM_FD", fdsc.to_string())
        .spawn()
        .expect("failed to run child process");

    child.wait().expect("child failed");

    clean_shared_memory(ptr, shm_id.as_str());
}
