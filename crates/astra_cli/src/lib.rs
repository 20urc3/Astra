use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about)]
pub struct Args {
    #[arg(short='i', long="input", help="The input folder with initial testcases")]
    pub input_folder: PathBuf,

    //#[arg(short='o', long="output")]
    //output_folder: PathBuf,

    //#[arg(short='t', long="timeout")]
    //timeout: u16,

    //#[arg(short='c', long="cores")]
    //cores: u16,

    //#[arg(short='t', long="target", help="target to fuzz")]
    //pub target: PathBuf,
    
    #[arg(short='p', long="program", help="Program to fuzz.")]
    pub program: PathBuf,
}