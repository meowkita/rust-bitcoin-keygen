use rustc_hash::FxHashSet;
use std::sync::{
    Arc,
    atomic::{AtomicU64, Ordering},
};

use crate::{address, keypair};

pub fn spawn(hashes: Arc<FxHashSet<[u8; 20]>>, counter: Arc<AtomicU64>) {
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
