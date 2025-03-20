use rustc_hash::FxHashSet;
use std::sync::{
    Arc,
    atomic::{AtomicBool, AtomicU64, Ordering},
};

use crate::{address, keypair};

pub fn register_graceful_shutdown(stop_flag: &Arc<AtomicBool>) {
    let stop_flag = Arc::clone(&stop_flag);

    ctrlc::set_handler(move || {
        println!("[ INFO] Received shutdown signal...");
        stop_flag.store(true, Ordering::Relaxed);
    })
    .expect("[ERROR] Failed to set a Ctrl+C handler");
}

pub fn spawn(
    thread_id: usize,
    hashes: Arc<FxHashSet<[u8; 20]>>,
    counter: Arc<AtomicU64>,
    stop: Arc<AtomicBool>,
) {
    while !stop.load(Ordering::Relaxed) {
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

    println!("[ INFO] THREAD-{}: Stopping...", thread_id);
}
