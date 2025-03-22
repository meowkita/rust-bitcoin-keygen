use bech32::{ToBase32, Variant};
use sha2::{Digest, Sha256};

mod test;

pub fn p2pkh(hash: &[u8; 20]) -> String {
    let mut address = vec![0x00];
    address.extend(hash);

    let checksum = &Sha256::digest(Sha256::digest(&address))[..4];
    address.extend(checksum);

    bs58::encode(address).into_string()
}

pub fn p2wpkh(hash: &[u8; 20]) -> String {
    let mut data = vec![bech32::u5::try_from_u8(0).unwrap()];
    data.extend_from_slice(&hash.to_base32());

    bech32::encode("bc", data, Variant::Bech32).unwrap()
}
