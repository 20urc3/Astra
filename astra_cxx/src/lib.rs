use std::env;
use std::process::{Command, exit};

// ------------------ UTILITY ------------------ 
/// Static sancov runtime archive, at the per-OS install location
/// used by `utils/install.sh`.
fn sancov_lib_path() -> &'static str {
    if cfg!(target_os = "macos") {
        "/usr/local/lib/libastra_sancov.a"
    } else {
        "/usr/lib/libastra_sancov.a"
    }
}

// ------------------ Compiler wrapper ------------------ 
/// This function is the main logic of the compiler wrapper
/// It provides arguments to clang, in order to instrument the target program 
/// This version is the C++ Version, effectively wrapping clang++
fn main() {
    let mut args: Vec<String> = env::args().skip(1).collect();
    let is_compilation = args.contains(&"-c".to_string());

    // Select sanitizers via the SANITIZE env var, e.g.
    //   SANITIZE=asan
    //   SANITIZE=ubsan
    //   SANITIZE=tsan
    //   SANITIZE=asan,ubsan   (ASan + UBSan combine; TSan is exclusive)
    let mut sanitizers: Vec<&str> = Vec::new();
    let san = env::var("SANITIZE").unwrap_or_default();
    for tok in san.split(',').map(str::trim).filter(|s| !s.is_empty()) {
        match tok.to_ascii_lowercase().as_str() {
            "asan" | "address" => sanitizers.push("address"),
            "ubsan" | "undefined" => sanitizers.push("undefined"),
            "tsan" | "thread" => sanitizers.push("thread"),
            other => {
                eprintln!("cc-wrapper: unknown sanitizer '{}'", other);
                exit(2);
            }
        }
    }

    if sanitizers.iter().any(|s| *s == "thread") && sanitizers.iter().any(|s| *s == "address") {
        eprintln!("cc-wrapper: TSan and ASan are mutually exclusive");
        exit(2);
    }

    let has_asan = sanitizers.iter().any(|s| *s == "address");

    let san_flag = if sanitizers.is_empty() {
        None
    } else {
        Some(format!("-fsanitize={}", sanitizers.join(",")))
    };

    if is_compilation {
        args.push("-fsanitize-coverage=trace-pc-guard".into());
        if let Some(flag) = &san_flag {
            args.push(flag.clone());
        }
        if has_asan {
            // Readable stack traces in ASan reports.
            args.push("-fno-omit-frame-pointer".into());
            args.push("-Og".into());
            args.push("-g".into());
        }
    } else {
        args.push("-fsanitize-coverage=trace-pc-guard".into());
        if let Some(flag) = &san_flag {
            args.push(flag.clone());
        }
        args.push("-Wl,--whole-archive,--allow-multiple-definition".into());
        args.push(sancov_lib_path().into());
        args.push("-Wl,--no-whole-archive".into());
    }

    let status = Command::new("clang++-20")
        .args(args)
        .status()
        .expect("failed to compile!");

    exit(status.code().unwrap_or(1));
}
