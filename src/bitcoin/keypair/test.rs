#[test]
fn test_generate() {
    let (secret_key, public_key) = crate::bitcoin::keypair::generate();

    assert!(!secret_key.display_secret().to_string().is_empty());
    assert!(!public_key.to_string().is_empty());
}
