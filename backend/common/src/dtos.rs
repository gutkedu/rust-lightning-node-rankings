use serde::{Deserialize, Serialize};

/// A struct representing a node in the dynamo database.
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

/// A struct representing a new node to be created in the dynamo database.
#[derive(Serialize, Deserialize, Debug)]
pub struct NewNodeDTO {
    pub channels: u64,
    pub public_key: String,
    pub alias: String,
    pub capacity_in_btc: String,
    pub first_seen_iso: String,
    pub updated_at: u64,
}
