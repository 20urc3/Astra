mod coverage;
mod mutators;
mod ui;
mod worker;

use chrono;
use colored_text::Colorize;
use ui::cli::*;
use worker::files::create_output_directory;
use worker::*;

use clap::Parser;

fn main() {
    // Parsing arguments
    let args = Args::parse();
    log_info!(
        "Astra-fuzz",
        "The target program to fuzz is {:?}",
        args.program
    );

    // File operations
    create_output_directory(&args.input_folder, &args.output_folder, args.resume);

    // Run workers
    running_workers(
        args.cores,
        args.output_folder,
        args.timeout,
        args.program,
        TargetArgs::parse(args.arguments),
        args.coverage.as_env().to_string(),
        args.resume,
        args.leak,
        args.sync,
    );
}
