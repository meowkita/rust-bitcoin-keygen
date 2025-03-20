use bs58::{self};
use ripemd::Ripemd160;
use secp256k1::PublicKey;
use sha2::{Digest, Sha256};

pub fn generate(public_key: &PublicKey) -> [u8; 20] {
    let public_bytes = public_key.serialize();
    let sha256_hash = Sha256::digest(public_bytes);
    let public_hash = Ripemd160::digest(sha256_hash);

    public_hash.try_into().unwrap()
}

pub fn to_hash(address: &str) -> Option<[u8; 20]> {
    let decoded = bs58::decode(address).into_vec().unwrap();

    if decoded.len() != 25 {
        eprintln!(
            "[ERROR] Hash length of the address {} is not 25 (actual {})",
            address,
            decoded.len()
        );

        return None;
    }

    Some(decoded[1..21].try_into().unwrap())
}

pub fn to_str(hash: [u8; 20]) -> String {
    let mut address = vec![0x00];
    address.extend(hash);

    let checksum = &Sha256::digest(Sha256::digest(&address))[..4];
    address.extend(checksum);

    bs58::encode(address).into_string()
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;
    use secp256k1::{PublicKey, Secp256k1, SecretKey, rand};

    #[test]
    fn test_generate_address() {
        let secp = Secp256k1::new();
        let secret_key = SecretKey::new(&mut rand::thread_rng());
        let public_key = PublicKey::from_secret_key(&secp, &secret_key);
        let address = generate(&public_key);
        assert!(address.len() == 20);
    }

    #[test]
    fn test_address_conversion() {
        let addresses = vec![
            "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa",
            "1dice8EMZmqKvrGE4Qc9bUFf9PX3xaYDp",
            "1BoatSLRHtKNngkdXEeobR76b53LETtpyT",
            "1FeexV6bAHb8ybZjqQMjJrcCrHGW9sb6uF",
        ];

        let mut hashes = vec![];
        for address in &addresses {
            let hash = to_hash(address).unwrap();
            hashes.push(hash);
        }

        let mut decoded_addresses = vec![];
        for hash in hashes {
            let decoded = to_str(hash);
            decoded_addresses.push(decoded);
        }

        assert_eq!(decoded_addresses, addresses);
    }

    #[test]
    fn test_address_conversion_2() {
        let sample_addresses = vec![
            "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa",
            "1BvBMSEYstWetqTFn5Au4m4GFg7xJaNVN2",
            "1P5ZEDWTKTFGxQjZphgWPQUpe554WKDfHQ",
        ];

        let mut hashes = HashSet::new();

        for &address in &sample_addresses {
            let hash = to_hash(address).unwrap();
            let reconstructed = to_str(hash);
            assert_eq!(reconstructed, address);
            hashes.insert(hash);
        }

        let secp = Secp256k1::new();
        for _ in 0..10 {
            let secret_key = SecretKey::new(&mut secp256k1::rand::thread_rng());
            let public_key = PublicKey::from_secret_key(&secp, &secret_key);
            let hash = generate(&public_key);

            assert!(hash != [0; 20]);
        }
    }
}
