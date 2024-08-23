use std::error::Error;

use serde::{Deserialize, Serialize};

/// The `NodeConnectivityRanking` struct represents the data structure returned by the Mempool API.
/// some fields have been omitted.
#[derive(Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct NodeConnectivityRanking {
    pub publicKey: String,
    pub alias: String,
    pub channels: u32,
    pub capacity: u64,
    pub firstSeen: u64,
    pub updatedAt: u64,
}

/// Fetch the node rankings from the Mempool API.
///  - The function returns a vector of `NodeConnectivityRanking` structs.
pub fn fetch_node_rankings() -> Result<Vec<NodeConnectivityRanking>, Box<dyn Error>> {
    const URL: &str = "https://mempool.space/api/v1/lightning/nodes/rankings/connectivity";

    let http_client = ureq::Agent::new();

    let response = http_client.get(URL).call()?;

    let res: Vec<NodeConnectivityRanking> = serde_json::from_str(&response.into_string()?)?;

    Ok(res)
}
