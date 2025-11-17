use std::collections::BTreeSet;
use std::path::PathBuf;
use std::fs;

pub fn collect_corpus(input_dir: &PathBuf) -> Vec<Vec<u8>> {
    let mut corpus_set = BTreeSet::new();

    for file in fs::read_dir(input_dir).unwrap() {
        let path = file.unwrap().path();
        if path.is_file() {
            let data = fs::read(&path).unwrap();
            corpus_set.insert(data);
        }
    }

    corpus_set.into_iter().collect::<Vec<Vec<u8>>>()

}
