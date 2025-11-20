// astra_cc.rs
use std::env;
use std::process::{Command, exit};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("Usage: astra_cc <args...>");
        exit(1);
    }

    let status = Command::new("clang-20")
        .args(&args)
        .env("CC", "clang-20")
        .env("LD", "clang-20")
        .env("CFLAGS", "-fsanitize-coverage=trace-pc-guard -fsanitize=address")
        .env("LDFLAGS", "-lastra_sancov -fsanitize=address")
        .status()
        .expect("failed to execute clang-20");

    exit(status.code().unwrap_or(1));
}
