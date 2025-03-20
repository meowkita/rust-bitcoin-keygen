use std::{
    env,
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicU64},
    },
    thread::{self},
};

mod address;
mod keypair;
mod storage;
mod worker;

fn main() {
    let threads_amount: usize = env::args()
        .nth(1)
        .and_then(|arg| arg.parse().ok())
        .unwrap_or(4);
    println!("[ INFO] Using {} threads.", threads_amount);

    let hashes = storage::load_hashes("data/bitcoin.tsv");
    let counter = Arc::new(AtomicU64::new(0));
    let stop = Arc::new(AtomicBool::new(false));
    worker::register_graceful_shutdown(&stop);

    println!("[ INFO] Starting key generation...");
    let handles: Vec<_> = (0..threads_amount)
        .map(|i| {
            let stop = Arc::clone(&stop);
            let hashes = Arc::clone(&hashes);
            let counter = Arc::clone(&counter);
            thread::spawn(move || worker::spawn(i, hashes, counter, stop))
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    println!("[ INFO] All threads stopped.");
}
