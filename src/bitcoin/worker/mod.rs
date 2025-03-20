use crate::{
    bitcoin::{address, hash, keypair},
    log_info,
};
use rustc_hash::FxHashSet;
use std::sync::{
    Arc,
    atomic::{AtomicBool, AtomicU64, Ordering},
};

pub fn register_graceful_shutdown(stop_flag: &Arc<AtomicBool>) {
    let stop_flag = Arc::clone(&stop_flag);

    ctrlc::set_handler(move || {
        log_info!("Received shutdown signal...");
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
        let hash = hash::from_key(&public_key).unwrap();

        let total = counter.fetch_add(1, Ordering::Relaxed) + 1;
        if total % 1_000_000 == 0 {
            println!("[ INFO] Total keys generated: {}m", total / 1_000_000);
        }

        if hashes.contains(&hash) {
            let p2pkh = address::p2pkh(&hash);
            let p2wpkh = address::p2wpkh(&hash);
            let private_key = secret_key.display_secret();

            log_info!("Found the key: {p2pkh} / {p2wpkh} - {private_key}");
        }
    }

    println!("[ INFO] Thread-{}: Stopping...", thread_id);
}
