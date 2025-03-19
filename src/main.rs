use rustc_hash::FxHashSet;
use std::{
    env,
    fs::OpenOptions,
    io::{BufRead, BufReader},
    sync::Arc,
    thread::{self},
};

mod address;
mod keygen;

fn main() {
    let threads_amount: usize = env::args()
        .nth(1)
        .and_then(|arg| arg.parse().ok())
        .unwrap_or(4);
    println!("[ INFO] Using {} threads.", threads_amount);

    let file = OpenOptions::new()
        .read(true)
        .open("data/bitcoin.tsv")
        .unwrap();
    let reader = BufReader::new(file);
    let mut addresses = FxHashSet::default();

    println!("[ INFO] Loading addresses from a file...");
    for line in reader.lines() {
        let line = line.unwrap();
        if line.starts_with("1") {
            addresses.insert(line);
        }
    }

    let addresses = Arc::new(addresses);

    println!("[ INFO] Generating private keys...");
    let mut tasks = Vec::new();
    for index in 0..threads_amount {
        let addresses = Arc::clone(&addresses);
        let task = thread::spawn(move || {
            let mut generated = 0;
            loop {
                let (secret_key, public_key) = keygen::generate_keypair();
                let address = address::generate_address(&public_key);

                generated = generated + 1;
                if generated % 100_000 == 0 {
                    println!(
                        "[ INFO] Thread-{}: generated {}m keys.",
                        index,
                        f64::from(generated) / 100_000.0 / 10.0
                    );
                }

                if addresses.contains(&address) {
                    println!(
                        "[ INFO] Found the key: {} - {}",
                        address,
                        secret_key.display_secret()
                    );
                }
            }
        });

        tasks.push(task);
    }

    for task in tasks {
        task.join().unwrap();
    }
}
