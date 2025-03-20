use std::{
    ptr::hash,
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicU64, Ordering},
    },
};

use rustc_hash::FxHashSet;

mod bitcoin;
mod macros;

fn main() {
    let hashes = match bitcoin::data::from() {
        Ok(hashes) => hashes,
        Err(error) => {
            log_error!("{error}");
            return;
        }
    };
}

fn spawn(
    thread_id: usize,
    hashes: Arc<FxHashSet<[u8; 20]>>,
    counter: Arc<AtomicU64>,
    stop: Arc<AtomicBool>,
) {
    while !stop.load(Ordering::Relaxed) {
        let (secret_key, public_key) = bitcoin::keypair::generate();
        let hash = bitcoin::hash::from_key(&public_key).unwrap();

        let total = counter.fetch_add(1, Ordering::Relaxed) + 1;
        if total % 1_000_000 == 0 {
            println!("[ INFO] Total keys generated: {}m", total / 1_000_000);
        }

        if hashes.contains(&hash) {}
    }

    println!("[ INFO] THREAD-{}: Stopping...", thread_id);
}
