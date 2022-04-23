use serde::{Deserialize, Serialize};
use rsa::*;
use crate::state::Entry;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Delta {
    pub entries : Vec<Entry>,
}

impl Delta {
    pub fn new(serialized : &[u8]) -> Self {
        rmps::from_slice(serialized).unwrap()
    }

    pub fn verify_with_public_key(&self, public_key: &RsaPublicKey) -> bool {
        self.entries.iter().all(|entry| entry.verify_with_public_key(public_key))
    }
}

impl Into<Vec<u8>> for Delta {
    fn into(self) -> Vec<u8> {
        rmps::to_vec(&self).unwrap()
    }
}