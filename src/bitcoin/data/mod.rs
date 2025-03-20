use crate::{bitcoin::hash, log_info};
use rustc_hash::FxHashSet;
use std::{
    fs::OpenOptions,
    io::{BufRead, BufReader},
    sync::Arc,
};

pub fn load() -> Result<Arc<FxHashSet<[u8; 20]>>, String> {
    let file = OpenOptions::new()
        .read(true)
        .open("data/bitcoin.tsv")
        .map_err(|_| "Failed to open 'data/bitcoin.tsv'")?;
    let buffer = BufReader::new(file);
    let mut hashes = FxHashSet::default();
    let mut skipped = 0;

    log_info!("Loading hashes from a file...");
    for line in buffer.lines().flatten() {
        if let Ok(address) = parse_line(&line) {
            hashes.insert(address);
            continue;
        }

        skipped = skipped + 1;
    }

    log_info!("Loaded {} (skipped {}).", &hashes.len(), skipped);
    Ok(Arc::new(hashes))
}

fn parse_line(line: &str) -> Result<[u8; 20], String> {
    let address = line.split("\t").nth(0).unwrap();
    hash::from_address(address)
}
