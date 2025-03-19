use bs58;
use ripemd::Ripemd160;
use secp256k1::PublicKey;
use sha2::{Digest, Sha256};

pub fn generate_address(public_key: &PublicKey) -> String {
    let public_bytes = public_key.serialize();
    let sha256_hash = Sha256::digest(public_bytes);
    let public_hash = Ripemd160::digest(sha256_hash);

    let mut address = vec![0x00];
    address.extend(public_hash);

    let checksum = &Sha256::digest(Sha256::digest(&address))[..4];
    address.extend(checksum);

    bs58::encode(address).into_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use secp256k1::{PublicKey, Secp256k1, SecretKey, rand};

    #[test]
    fn test_generate_address() {
        let secp = Secp256k1::new();
        let secret_key = SecretKey::new(&mut rand::thread_rng());
        let public_key = PublicKey::from_secret_key(&secp, &secret_key);
        let address = generate_address(&public_key);
        assert!(address.len() > 20);
    }
}
