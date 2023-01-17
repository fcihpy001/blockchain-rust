use serde::{Serialize, Deserialize};
use ring::signature::{EcdsaKeyPair, ECDSA_P256_SHA256_FIXED_SIGNING, KeyPair};
use crate::utils::{checksum, hash_pub_key};
use crate::utils::secret::{base58_encode, new_private_key, ripemd160_digest, sha256_digest};

const VERSION: u8 = 0x00;
pub const ADDRESS_CHECKSUM_LEN: usize = 4;

#[derive(Serialize, Deserialize, Clone)]
pub struct Wallet {
    pkcs8: Vec<u8>,
    public_key: Vec<u8>
}

impl Wallet {
    pub fn new() -> Self {
        let pkcs8 = new_private_key();
        let key_pair = EcdsaKeyPair::from_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, pkcs8.as_ref()).unwrap();
        let public_key = key_pair.public_key().as_ref().to_vec();
        Self {
            pkcs8,
            public_key
        }
    }

    pub fn get_address(&self) -> String {
        let pub_key_hash = hash_pub_key(self.public_key.as_slice());
        let mut payload = vec![];
        payload.push(VERSION);
        payload.extend(pub_key_hash.as_slice());

        let checksum = checksum(payload.as_slice());
        payload.extend(checksum.as_slice());
        base58_encode(payload.as_slice())
    }

    pub fn get_pkcs8(&self) -> &[u8] {
        self.pkcs8.as_slice()
    }

    pub fn get_public_key(&self) -> &[u8] {
        self.public_key.as_slice()
    }
}
