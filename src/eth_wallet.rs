use serde::{Deserialize, Serialize};

use secp256k1::{
    Secp256k1, PublicKey, SecretKey,
};

use web3::{types::{Address}};

use tiny_keccak::keccak256;

use num_bigint::BigUint;

#[derive(Serialize, Deserialize, Debug)]
pub struct Wallet {
    pub secret_key: String,
    pub public_key: String,
    pub public_address: String,
}

pub fn generate_keypair(big_int: &num_bigint::BigUint) -> Option<Wallet> {
    let secp = Secp256k1::new();
    if let Some(secret_key) = private_from_big(big_int) {
        let public_key = PublicKey::from_secret_key(&secp, &secret_key);
        let addr: Address = public_key_address(&public_key);

        return Some(Wallet{
            secret_key: format!("{}", secret_key.to_string()),
            public_key: public_key.to_string(),
            public_address: format!("{:?}", addr),
        });

    }
    
    return None;
}

pub fn public_key_address(public_key: &PublicKey) -> Address {
    let public_key = public_key.serialize_uncompressed();
    debug_assert_eq!(public_key[0], 0x04);
    let hash = keccak256(&public_key[1..]);

    Address::from_slice(&hash[12..])
}

pub fn private_from_big(num: &BigUint) -> Option<SecretKey>{
	match SecretKey::from_slice(&fill_vec(&num.to_bytes_be()) )  {
		Ok(secret_key) => {
	        return Some(secret_key);
		},
		Err(err) => {
            println!("{:?}", err);
            return None;
		}
	}
}

pub fn fill_vec(data : &[u8]) -> [u8; 32] {
	let mut result = [0u8; 32];
	let mut i = 0;
	for n in data {
		if i == 32 {
			break
		}
		result[i] = *n;
		i = i + 1;
	}
	return result;
}

pub fn str_to_big(str : String, radix: u32) -> BigUint{
    BigUint::parse_bytes(&(*str.into_bytes()), radix).unwrap()
 }