use rustc_hash::FxHashSet;
use std::io::BufRead;
use std::{fs::OpenOptions, io::BufReader, sync::Arc};

use crate::address;

pub fn load_hashes(path: &str) -> Arc<FxHashSet<[u8; 20]>> {
    let file = OpenOptions::new().read(true).open(&path).unwrap();
    let reader = BufReader::new(file);
    let mut hashes = FxHashSet::default();
    let mut skipped = 0;

    println!("[ INFO] Loading hashes from a file...");
    for line in reader.lines().flatten() {
        if line.starts_with("1") {
            let address = line.split("\t").nth(0).unwrap().trim();
            if let Some(hash) = address::to_hash(&address) {
                hashes.insert(hash);
            } else {
                skipped = skipped + 1;
            }
        }
    }

    println!("[ INFO] Loaded {} (skipped {}).", &hashes.len(), skipped);
    Arc::new(hashes)
}
