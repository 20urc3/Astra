use crate::log_info;
use colored_text::Colorize;
use std::collections::BTreeSet;
use std::collections::HashSet;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub type Corpus = Vec<Vec<u8>>;

/// Path to a worker's private corpus directory.
pub fn node_dir(output: &Path, id: u16) -> PathBuf {
    output.join(format!("node_{id}"))
}

/// Prepare a worker's corpus: create its node directory and build the
/// initial pool (shared seeds, plus its own prior finds when resuming).
/// Returns `(node_folder, initial_pool, next_file_index)`.
pub fn init_worker_corpus(
    output: &Path,
    id: u16,
    resume: bool,
    shared_seeds: &Corpus,
) -> io::Result<(PathBuf, Corpus, usize)> {
    let node_folder = node_dir(output, id);
    fs::create_dir_all(&node_folder)?;

    let mut pool = shared_seeds.clone();
    let mut next_index = 0;
    if resume {
        let prior = load_corpus(&node_folder)?;
        next_index = prior.len();
        pool.extend(prior);
    }

    Ok((node_folder, pool, next_index))
}

/// Pull in inputs discovered by *other* workers since the last sync.
/// Scans sibling `node_*` directories, reading any file not already seen.
/// `seen` persists across calls so each sibling find is imported once.
pub fn sync_corpus(output: &Path, own_id: u16, seen: &mut HashSet<PathBuf>) -> io::Result<Corpus> {
    let mut fresh = Corpus::new();
    let own = node_dir(output, own_id);

    for entry in fs::read_dir(output)? {
        let dir = entry?.path();
        if !dir.is_dir() || dir == own {
            continue; // only sibling dirs, skip our own
        }
        let name = dir.file_name().and_then(|n| n.to_str()).unwrap_or("");
        if !name.starts_with("node_") {
            continue;
        }

        for file in fs::read_dir(&dir)? {
            let path = file?.path();
            // `insert` returns true only for a path we haven't imported yet.
            if path.is_file() && seen.insert(path.clone()) {
                if let Ok(data) = fs::read(&path) {
                    fresh.push(data);
                }
            }
        }
    }
    Ok(fresh)
}

pub fn create_output_directory(input_folder: &PathBuf, output_folder: &PathBuf, resume: bool) {
    // If not resuming, remove any existing output first
    if !resume && output_folder.exists() {
        if let Err(error) = std::fs::remove_dir_all(&output_folder) {
            eprintln!(
                "Couldn't clear the output directory: {} : {error}",
                output_folder.display()
            );
            return;
        }
    }

    // Create output dir
    if let Err(error) = std::fs::create_dir_all(&output_folder) {
        eprintln!(
            "Couldn't create the output directory: {} : {error}",
            output_folder.display()
        );
        return;
    }

    match collect_seeds(&input_folder, &output_folder) {
        Ok(()) => {
            log_info!(
                "Astra-fuzz",
                "Using output folder: {}",
                output_folder.display()
            );
        }
        Err(error) => eprintln!("failed to collect seeds: {error}"),
    }
}

pub fn collect_seeds(input_dir: &PathBuf, output_path: &PathBuf) -> io::Result<()> {
    // Read + dedup input seeds
    let mut corpus_set = BTreeSet::new();
    for file in fs::read_dir(input_dir)? {
        let path = file?.path();
        if path.is_file() {
            corpus_set.insert(fs::read(&path)?);
        }
    }

    // Create output_path/seeds/
    let seeds_dir = output_path.join("seeds");
    fs::create_dir_all(&seeds_dir)?;

    // Write each one out
    for (i, data) in corpus_set.into_iter().enumerate() {
        let name = format!("{:09}", i);
        fs::write(seeds_dir.join(name), data)?;
    }

    Ok(())
}

// Load corpus in memory
pub fn load_corpus(dir: &Path) -> io::Result<Corpus> {
    let mut corpus = Corpus::new();

    let entries = fs::read_dir(dir)?;

    for entry in entries {
        let entry = entry?;

        let path = entry.path();
        if path.is_file() {
            let data = fs::read(&path)?;
            corpus.push(data);
        }
    }

    Ok(corpus)
}
