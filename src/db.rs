use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;

const FILENAME: &str = "/Users/hrodri20/Documents/eth/rust-crypto-wallet/privates/";

pub fn store(private_key: String, address: String) -> std::io::Result<()> {
	let mut file = File::create(format!("{}{}.tx", FILENAME, &address.to_string()))?;
	let content_string = format!("'private_key': '{}', 'address': '{}'", &private_key.to_string(), &address.to_string());
    return write!(file, "{}", content_string);
}

pub fn exist_address(list: &HashSet<String>, address: &String) -> bool {
    return list.contains(address);
}