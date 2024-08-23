use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Node {
    pub pk: String,
    pub sk: String,
    pub lsi1: u64,
    pub channels: u64,
    pub public_key: String,
    pub alias: String,
    pub capacity: String,
    pub first_seen: String,
    pub updated_at: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewNodeDTO {
    pub channels: u64,
    pub public_key: String,
    pub alias: String,
    pub capacity_in_btc: String,
    pub first_seen_iso: String,
    pub updated_at: u64,
}
