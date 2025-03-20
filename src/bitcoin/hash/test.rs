#[test]
fn test_p2pkh_conversion() {
    let address = "1FeexV6bAHb8ybZjqQMjJrcCrHGW9sb6uF";
    let hash = crate::bitcoin::hash::from_address(address).unwrap();
    assert_eq!(hash.len(), 20);
}

#[test]
fn test_p2wpkh_conversion() {
    let address = "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh";
    let hash = crate::bitcoin::hash::from_address(address).unwrap();
    assert_eq!(hash.len(), 20);
}

#[test]
fn test_valid_key_conversion() {
    let secp = secp256k1::Secp256k1::new();
    let secret_key = secp256k1::SecretKey::new(&mut secp256k1::rand::thread_rng());
    let public_key = secp256k1::PublicKey::from_secret_key(&secp, &secret_key);

    let result = crate::bitcoin::hash::from_key(&public_key);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 20);
}
