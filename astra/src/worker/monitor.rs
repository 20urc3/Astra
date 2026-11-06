use std::fs;
use std::path::Path;

/// Context captured at crash time, used for dedup and triage.
pub struct CrashInfo<'a> {
    pub signal: i32,
    pub report: &'a str, // child stderr (ASan/UBSan text); may be empty
    pub argv: &'a [String],
}

/// Collapse the "same bug, thousands of inputs" spam into one file:
/// derive the key from the sanitizer stack trace, not the input bytes.
fn crash_signature(signal: i32, report: &str, testcase: &[u8]) -> String {
    let frames: Vec<&str> = report
        .lines()
        .map(str::trim_start)
        .filter(|l| l.starts_with('#'))
        .take(4)
        .collect();

    let material = if frames.is_empty() {
        // No report (target not sanitizer-built): fall back to signal + input.
        format!("sig{signal}:{}", sha256::digest(testcase))
    } else {
        format!("sig{signal}:{}", frames.join("\n"))
    };
    sha256::digest(material.as_bytes())[..16].to_string()
}

/// Saves a crashing input (plus a .meta sidecar) under a signature-based
/// name so one bug => one file, however many inputs reach it.
pub fn record_crash(output_folder_path: &Path, testcase: &[u8], info: &CrashInfo) {
    let crashes_dir = output_folder_path.join("crashes");
    if let Err(error) = fs::create_dir_all(&crashes_dir) {
        eprintln!(
            "failed to create crashes dir {}: {error}",
            crashes_dir.display()
        );
        return;
    }

    let sig = crash_signature(info.signal, info.report, testcase);
    let path = crashes_dir.join(format!("crash_{sig}"));

    // First input to hit this bug wins; later ones are the same bug.
    if path.exists() {
        return;
    }

    if let Err(error) = fs::write(&path, testcase) {
        eprintln!("failed to write crash {}: {error}", path.display());
        return;
    }

    let meta = format!(
        "signal: {}\nargv: {}\n\n--- report ---\n{}\n",
        info.signal,
        info.argv.join(" "),
        info.report,
    );
    if let Err(error) = fs::write(path.with_extension("meta"), meta) {
        eprintln!("failed to write crash metadata: {error}");
    }
}
