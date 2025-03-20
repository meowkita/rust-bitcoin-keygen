use bech32::FromBase32;
use ripemd::Ripemd160;
use secp256k1::PublicKey;
use sha2::{Digest, Sha256};

mod test;

pub fn from_key(public_key: &PublicKey) -> Result<[u8; 20], String> {
    let public_bytes = public_key.serialize();
    let sha256_hash = Sha256::digest(public_bytes);
    let public_hash = Ripemd160::digest(sha256_hash);

    Ok(public_hash
        .try_into()
        .map_err(|_| "Address decode error.")?)
}

pub fn from_address(address: &str) -> Result<[u8; 20], String> {
    if address.starts_with("1") {
        return p2pkh(address);
    }

    if address.starts_with("bc1q") {
        return p2wpkh(address);
    }

    Err(String::from("Unknown address type."))
}

fn p2pkh(address: &str) -> Result<[u8; 20], String> {
    let decoded = bs58::decode(address)
        .into_vec()
        .map_err(|_| "Address decode error.")?;

    let hash = decoded[1..21]
        .try_into()
        .map_err(|_| "Can't exctract hash from a vector")?;

    return Ok(hash);
}

fn p2wpkh(address: &str) -> Result<[u8; 20], String> {
    let decoded = bech32::decode(address)
        .map_err(|_| "Address decode error.")?
        .1;

    let slice = &decoded[1..];
    let hash = Vec::from_base32(&slice)
        .map_err(|_| "Can't map decoded slice to bytes.")?
        .try_into()
        .map_err(|_| format!("Invalid hash length: expected 20, got {}", slice.len()))?;

    return Ok(hash);
}
