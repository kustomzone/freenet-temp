use std::{borrow::Borrow};
use serde::{Deserialize, Serialize};
use rsa::*;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct State {
    pub entries : Vec<Entry>,
}

impl State {
    pub fn verify_with_public_key(&self, public_key: &RsaPublicKey) -> bool {
        for (i, entry) in self.entries.iter().enumerate() {
            if i as u32 != entry.get_content().index {
                return false;
            }
        }
        self.entries.iter().all(|entry| entry.verify_with_public_key(public_key))
    }
}

impl From<&[u8]> for State {
    fn from(serialized : &[u8]) -> Self {
        rmps::from_slice(serialized).unwrap()
    }
}

impl From<Vec<u8>> for State {
    fn from(serialized : Vec<u8>) -> Self {
        rmps::from_slice(serialized.as_slice()).unwrap()
    }
}

impl Into<Vec<u8>> for State {
    fn into(self) -> Vec<u8> {
        rmps::to_vec(&self).unwrap()
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Entry {
    pub content : Vec<u8>,
    pub signature : Vec<u8>,
}

impl Entry {
    pub fn verify_with_public_key(&self, signer : &RsaPublicKey) -> bool {
        signer.verify(
            PaddingScheme::new_pkcs1v15_sign(Option::None), 
            self.content.borrow(), 
            self.signature.borrow()).is_ok()
    }

    pub fn get_content(&self) -> Content {
        rmps::from_slice(self.content.as_slice()).unwrap()
    }
}

impl Into<Vec<u8>> for Entry {
    fn into(self) -> Vec<u8> {
        rmps::to_vec(&self).unwrap()
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Content {
    pub index : u32,
    pub data : Vec<u8>,
}

impl Content {
    pub fn new(serialized : &[u8]) -> Self {
        rmps::from_slice(serialized).unwrap()
    }
}

impl Into<Vec<u8>> for Content {
    fn into(self) -> Vec<u8> {
        rmps::to_vec(&self).unwrap()
    }
}