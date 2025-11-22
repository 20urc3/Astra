use astra_cli::*;
use astra_worker::*;
use clap::Parser;

fn main() {
    // Parsing arguments
    let args = Args::parse();
    println!("You passed the program to link: {:?}", args.program);

    // Run workers
    running_workers(10,args.input_folder, args.program, args.arguments);
}
