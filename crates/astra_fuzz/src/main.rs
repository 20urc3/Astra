use astra_cli::*;
use astra_worker::*;
use astra_tui::log_info;
use colored_text::Colorize;
use chrono;

use clap::Parser;

fn main() {
    // Parsing arguments
    let args = Args::parse();
    log_info!("Astra-fuzz", "The target program to fuzz is {:?}", args.program);

    // Run workers
    running_workers(10,args.input_folder, args.timeout, args.program, args.arguments);
}
