use std::path::PathBuf;
use std::process::Command;

pub fn linking_target_to_sancov(target_path: &PathBuf) {
    let mut child = Command::new("clang-20")
        .arg(target_path)
        .args([
            "-Wl,--whole-archive,--allow-multiple-definition", 
            "./target/release/libastra_sancov.a", 
            "-fsanitize=address"])
        .spawn()
        .expect("Failed to run linking with clang");
    child.wait().unwrap();
}