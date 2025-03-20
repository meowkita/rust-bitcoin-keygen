use bech32::{ToBase32, Variant, u5};
use sha2::{Digest, Sha256};

mod test;

pub fn to_p2pkh(hash: [u8; 20]) -> String {
    let mut address = vec![0x00];
    address.extend(hash);

    let checksum = &Sha256::digest(Sha256::digest(&address))[..4];
    address.extend(checksum);

    bs58::encode(address).into_string()
}
pub fn to_p2wpkh(hash: [u8; 20]) -> String {
    let mut data = vec![0u8];
    data.extend_from_slice(&hash);

    let hash5 = data.to_base32();
    bech32::encode("bc", hash5, Variant::Bech32).unwrap()
}
