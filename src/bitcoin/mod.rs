use crate::log_info;
use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicU64},
    },
    thread,
};

pub mod address;
pub mod data;
pub mod hash;
pub mod keypair;
pub mod worker;

pub fn init(threads_amount: usize, max_hashes: u64) -> Result<(), String> {
    let hashes = data::load(max_hashes)?;
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

    Ok(())
}
