use astra_cli::*;
use astra_tui::log_info;
use astra_worker::*;
use chrono;
use colored_text::Colorize;

use clap::Parser;

fn main() {
    // Parsing arguments
    let args = Args::parse();
    log_info!(
        "Astra-fuzz",
        "The target program to fuzz is {:?}",
        args.program
    );

    // Run workers
    running_workers(
        args.cores,
        args.input_folder,
        args.timeout,
        args.program,
        args.arguments,
    );
}
