#![allow(unused)]

use bitcoin_arch_v2::{Address, CompressedPublicKey, Network, PublicKey};
use std::error::Error;


pub fn p2wpkh_address(public_key: &String) -> Result<String, Box<dyn Error>> {
    let decoded: Vec<u8> = hex::decode(public_key)?;
    let public_key = PublicKey::from_slice(&decoded)?;
    let comp_pk = CompressedPublicKey(public_key.inner); // p2wpkh requires compressed public key
    let address_p2wpkh = Address::p2wpkh(&comp_pk, Network::Bitcoin);

    Ok(address_p2wpkh.to_string())
}

pub fn p2pkh_address(public_key: &String) -> Result<String, Box<dyn Error>> {
    let decoded: Vec<u8> = hex::decode(public_key)?;
    let public_key = PublicKey::from_slice(&decoded)?;
    let address_p2pkh = Address::p2pkh(&public_key, Network::Bitcoin);

    Ok(address_p2pkh.to_string())
}

