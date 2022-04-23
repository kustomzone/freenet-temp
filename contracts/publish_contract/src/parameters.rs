use serde::{Deserialize, Serialize};
use rsa::*;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Parameters {
    pub public_key : RsaPublicKey,
}