use secp256k1::{PublicKey, Secp256k1, SecretKey, rand};

mod test;

pub fn generate() -> (SecretKey, PublicKey) {
    let secp = Secp256k1::new();
    let secret_key = SecretKey::new(&mut rand::thread_rng());
    let public_key = PublicKey::from_secret_key(&secp, &secret_key);

    (secret_key, public_key)
}
