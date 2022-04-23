use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct StateSummary {
    pub next_index : u32,
}

impl StateSummary {
    pub fn new(serialized : &[u8]) -> Self {
        rmps::from_slice(serialized).unwrap()
    }
}

impl Into<Vec<u8>> for StateSummary {
    fn into(self) -> Vec<u8> {
        rmps::to_vec(&self).unwrap()
    }
}