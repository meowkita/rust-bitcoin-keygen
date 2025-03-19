use std::{
    collections::HashSet,
    fs::OpenOptions,
    io::{BufRead, BufReader},
};

mod address;
mod keygen;

fn main() {
    let file = OpenOptions::new()
        .read(true)
        .open("data/bitcoin.tsv")
        .unwrap();
    let reader = BufReader::new(file);
    let mut addresses = HashSet::new();

    println!("Load addresses from a file...");
    for line in reader.lines() {
        let line = line.unwrap();
        if line.starts_with("1") {
            addresses.insert(line);
        }
    }

    println!("Generating private keys...");
    let mut generated = 0;
    loop {
        let (secret_key, public_key) = keygen::generate_keypair();
        let address = address::generate_address(&public_key);

        generated = generated + 1;
        if generated % 100_000 == 0 {
            println!(
                "Generated {}m keys.",
                f64::from(generated) / 100_000.0 / 10.0
            );
        }

        if addresses.contains(&address) {
            println!(
                "Found the key: {} - {}",
                address,
                secret_key.display_secret()
            );
        }
    }
}
