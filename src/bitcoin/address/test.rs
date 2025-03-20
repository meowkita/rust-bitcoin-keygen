#[test]
fn test_p2pkh_and_p2wpkh_conversion() {
    let secp = secp256k1::Secp256k1::new();
    let secret_key = secp256k1::SecretKey::new(&mut secp256k1::rand::thread_rng());
    let public_key = secp256k1::PublicKey::from_secret_key(&secp, &secret_key);

    let public_hash = crate::bitcoin::hash::from_key(&public_key).unwrap();
    let p2pkh_address = crate::bitcoin::address::p2pkh(&public_hash);
    let p2wpkh_address = crate::bitcoin::address::p2wpkh(&public_hash);

    assert!(
        p2pkh_address.starts_with("1"),
        "P2PKH должен начинаться с '1'"
    );

    assert!(
        p2wpkh_address.starts_with("bc1q"),
        "P2WPKH должен начинаться с 'bc1q'"
    );
}
