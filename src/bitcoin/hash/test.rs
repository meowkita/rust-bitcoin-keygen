#[test]
fn test_p2pkh_conversion() {
    let address = "1FeexV6bAHb8ybZjqQMjJrcCrHGW9sb6uF";
    let hash = crate::bitcoin::hash::from(address).unwrap();
    assert_eq!(hash.len(), 20);
}

#[test]
fn test_p2wpkh_conversion() {
    let address = "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh";
    let hash = crate::bitcoin::hash::from(address).unwrap();
    assert_eq!(hash.len(), 20);
}
