use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about, trailing_var_arg=true)]
pub struct Args {
    #[arg(short='i', long="input", help="Specify the input folder path containing initial testcases.")]
    pub input_folder: PathBuf,

    #[arg(short='o', long="output", default_value="output", help="Specify the output folder where crashing or hanging inputs are saved.")]
    pub output_folder: PathBuf,

    #[arg(short='t', long="timeout", default_value="10", help="Specify the number of ms before considering the target program has timeout.")]
    pub timeout: u64,

    #[arg(short='c', long="cores", default_value="4", help="Specify the number of core used for parallel fuzzing.")]
    pub cores: u16,
    
    #[arg(short='p', long="program", help="Specify the program to fuzz.")]
    pub program: PathBuf,

    #[arg(trailing_var_arg = true, allow_hyphen_values = true, help="Specify the arguments passed to the target program.")]
    pub arguments: Vec<String>,
}
