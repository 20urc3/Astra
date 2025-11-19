use std::fs;
use std::io::prelude::*;
use std::hash::Hash;

/// Saves crashing unique input to a crash folder 
pub fn record_crash(input: Vec<u8>) {
    fs::create_dir_all("./output/crashes").unwrap();
    let unique_hash = sha256::digest(&input);
    let filename = format!("./output/crashes/crash_{}", unique_hash);
    let mut file = fs::File::create(filename).unwrap();
    file.write_all(&input).unwrap();   
}
