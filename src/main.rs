use rustc_hash::FxHashSet;
use std::{
    env,
    fs::OpenOptions,
    io::{BufRead, BufReader},
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
    thread::{self},
};

mod address;
mod keypair;

fn main() {
    let threads_amount: usize = env::args()
        .nth(1)
        .and_then(|arg| arg.parse().ok())
        .unwrap_or(4);
    println!("[ INFO] Using {} threads.", threads_amount);

    let hashes = load_hashes("data/bitcoin.tsv");
    let counter = Arc::new(AtomicU64::new(0));

    println!("[ INFO] Starting key generation...");
    let handles: Vec<_> = (0..threads_amount)
        .map(|_| {
            let hashes = Arc::clone(&hashes);
            let counter = Arc::clone(&counter);
            thread::spawn(move || spawn(hashes, counter))
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }
}

fn load_hashes(path: &str) -> Arc<FxHashSet<[u8; 20]>> {
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

fn spawn(hashes: Arc<FxHashSet<[u8; 20]>>, counter: Arc<AtomicU64>) {
    loop {
        let (secret_key, public_key) = keypair::generate();
        let hash = address::generate(&public_key);

        let total = counter.fetch_add(1, Ordering::Relaxed) + 1;
        if total % 1_000_000 == 0 {
            println!("[ INFO] Total keys generated: {}m", total / 1_000_000);
        }

        if hashes.contains(&hash) {
            println!(
                "[ INFO] Found the key: {} - {}",
                address::to_str(hash),
                secret_key.display_secret()
            );
        }
    }
}
