use crate::log_info;
use clap::Parser;
use colored_text::Colorize;
use std::path::PathBuf;
use std::thread::available_parallelism;

/// Target argv template parsed from the trailing CLI args.
/// Owns all `@@` substitution so the worker stays dumb.
#[derive(Clone)]
pub struct TargetArgs {
    template: Vec<String>,
    has_token: bool,
}

impl TargetArgs {
    pub fn parse(arguments: Vec<String>) -> Self {
        let has_token = arguments.iter().any(|a| a.contains("@@"));
        Self {
            template: arguments,
            has_token,
        }
    }

    /// Concrete argv for one execution, input path filled in.
    pub fn render(&self, input_path: &str) -> Vec<String> {
        if !self.has_token {
            let mut out = self.template.clone();
            out.push(input_path.to_string());
            return out;
        }
        self.template
            .iter()
            .map(|a| a.replace("@@", input_path))
            .collect()
    }
}

fn verify_seed_folder(s: &str) -> Result<PathBuf, String> {
    let folder = PathBuf::from(s);
    log_info!("Astra-CLI", "Verifiying input folder {s}");

    if !folder.is_dir() {
        return Err(format!("The input folder does not exist: {s}"));
    }

    let entries: Vec<_> = std::fs::read_dir(&folder)
        .map_err(|e| format!("cannot read input folder {s}: {e}"))?
        .filter_map(Result::ok)
        .collect();

    let has_seed = entries.iter().any(|entry| entry.path().is_file());

    let seed_count = entries.iter().filter(|e| e.path().is_file()).count();
    log_info!("Astra-CLI", "Discovered {seed_count} initial inputs");

    if has_seed {
        Ok(folder)
    } else {
        Err("input folder doesn't contain any seed file".to_string())
    }
}

fn verify_core(s: &str) -> Result<u16, String> {
    let cores: u16 = s
        .parse()
        .map_err(|_| format!("`{s}` is not a valid core count"))?;

    if cores == 0 {
        return Err("core count must be at least 1".to_string());
    }

    let available = available_parallelism().map(|n| n.get()).unwrap_or(1);

    if cores as usize > available {
        return Err(format!(
            "requested {cores} cores but only {available} are available"
        ));
    }

    Ok(cores)
}

#[derive(clap::ValueEnum, Clone, Copy, Debug)]
pub enum CoverageMode {
    /// AFL-style edge (prev -> cur transition) coverage.
    Edge,
    /// Basic-block coverage.
    Block,
}

impl CoverageMode {
    /// Value passed to the instrumented target via the ASTRA_COV_MODE env var.
    pub fn as_env(&self) -> &'static str {
        match self {
            CoverageMode::Edge => "edge",
            CoverageMode::Block => "block",
        }
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about, trailing_var_arg = true)]
pub struct Args {
    #[arg(
        short = 'i',
        long = "input",
        value_parser = verify_seed_folder,
        help = "Specify the input folder path containing initial testcases."
    )]
    pub input_folder: PathBuf,

    #[arg(
        short = 'o',
        long = "output",
        default_value = "output",
        help = "Specify the output folder of your fuzzing run."
    )]
    pub output_folder: PathBuf,

    #[arg(
        short = 't',
        long = "timeout",
        default_value = "5000",
        help = "Specify the number of ms before considering the target program has timeout."
    )]
    pub timeout: u64,

    #[arg(
        short = 'c',
        long = "cores",
        value_parser = verify_core,
        default_value = "4",
        help = "Specify the number of core used for parallel fuzzing."
    )]
    pub cores: u16,

    #[arg(short = 'p', long = "program", help = "Specify the program to fuzz.")]
    pub program: PathBuf,

    #[arg(
        short = 'r',
        long = "resume",
        default_value = "false",
        help = "resume campaign from previous output folder"
    )]
    pub resume: bool,

    #[arg(
        long = "coverage",
        value_enum,
        default_value = "edge",
        help = "Coverage granularity: edge (AFL-style transitions) or block."
    )]
    pub coverage: CoverageMode,

    #[arg(
        short = 'l',
        long = "leak",
        default_value = "false",
        help = "Enable LeakSanitizer leak detection (disabled by default)."
    )]
    pub leak: bool,

    #[arg(
        short = 'S',
        long = "sync",
        default_value = "false",
        help = "Periodically import other workers' finds from the output folder (disabled by default)."
    )]
    pub sync: bool,

    #[arg(
        trailing_var_arg = true,
        allow_hyphen_values = true,
        help = "Specify the arguments passed to the target program."
    )]
    pub arguments: Vec<String>,
}
