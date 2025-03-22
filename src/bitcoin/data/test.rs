#[test]
fn test_load_p2wpkh() {
    let (_, public_key) = crate::bitcoin::keypair::generate();
    let hash = crate::bitcoin::hash::from_key(&public_key).unwrap();
    let address = crate::bitcoin::address::p2wpkh(&hash);

    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open("test_bitcoin.tsv")
        .unwrap();

    let address = format!("{address}\t32301203");
    std::io::Write::write_all(&mut file, address.as_bytes()).unwrap();

    let hashes = crate::bitcoin::data::load("test_bitcoin.tsv", 100).unwrap();
    let contains = hashes.contains(&hash);

    assert!(contains);

    std::fs::remove_file("test_bitcoin.tsv").unwrap();
}

#[test]
fn test_load_p2pkh() {
    let (_, public_key) = crate::bitcoin::keypair::generate();
    let hash = crate::bitcoin::hash::from_key(&public_key).unwrap();
    let address = crate::bitcoin::address::p2pkh(&hash);

    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open("test_bitcoin2.tsv")
        .unwrap();

    let address = format!("{address}\t32301203");
    std::io::Write::write_all(&mut file, address.as_bytes()).unwrap();

    let hashes = crate::bitcoin::data::load("test_bitcoin2.tsv", 100).unwrap();
    let contains = hashes.contains(&hash);

    assert!(contains);

    std::fs::remove_file("test_bitcoin2.tsv").unwrap();
}
