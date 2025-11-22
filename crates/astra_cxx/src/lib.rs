use std::env;
use std::process::{Command, exit};

fn main() {
    let mut args: Vec<String> = env::args().skip(1).collect();
    let is_compilation = args.contains(&"-c".to_string());

    if is_compilation {
        args.push("-fsanitize-coverage=trace-pc-guard".into());
    } else {
        args.push("-fsanitize-coverage=trace-pc-guard".into());
        args.push("-Wl,--whole-archive,--allow-multiple-definition".into());
        args.push("/usr/lib/libastra_sancov.a".into());
        args.push("-Wl,--no-whole-archive".into());
    }

    let status = Command::new("clang-20")
        .args(args)
        .status()
        .expect("failed to compile!");

    exit(status.code().unwrap_or(1));
}
