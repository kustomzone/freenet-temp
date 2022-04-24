use serde::{Deserialize, Serialize};
use rsa::*;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Parameters {
    pub public_key : RsaPublicKey,
}


impl Into<Vec<u8>> for Parameters {
    fn into(self) -> Vec<u8> {
        rmps::to_vec(&self).unwrap()
    }
}

impl From<&[u8]> for Parameters {
    fn from(serialized : &[u8]) -> Self {
        rmps::from_slice(serialized).unwrap()
    }
}

