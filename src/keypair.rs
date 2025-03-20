use secp256k1::{PublicKey, Secp256k1, SecretKey, rand};

pub fn generate() -> (SecretKey, PublicKey) {
    let secp = Secp256k1::new();
    let secret_key = SecretKey::new(&mut rand::thread_rng());
    let public_key = PublicKey::from_secret_key(&secp, &secret_key);

    (secret_key, public_key)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate() {
        let (secret_key, public_key) = generate();

        assert!(!secret_key.display_secret().to_string().is_empty());
        assert!(!public_key.to_string().is_empty());
    }
}
