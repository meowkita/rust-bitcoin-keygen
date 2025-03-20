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
mod macros;
mod storage;
mod worker;

fn main() {
    let threads_amount: usize = env::args()
        .nth(1)
        .and_then(|arg| arg.parse().ok())
        .unwrap_or(4);
    log_info!("Using {} threads.", threads_amount);

    let hashes = storage::load_hashes("data/bitcoin.tsv");
    let counter = Arc::new(AtomicU64::new(0));
    let stop = Arc::new(AtomicBool::new(false));
    worker::register_graceful_shutdown(&stop);

    log_info!("Starting key generation...");
    let mut handles = Vec::new();
    for i in 0..threads_amount {
        let stop = Arc::clone(&stop);
        let hashes = Arc::clone(&hashes);
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || worker::spawn(i, hashes, counter, stop));

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    log_info!("All threads has stopped.");
}
