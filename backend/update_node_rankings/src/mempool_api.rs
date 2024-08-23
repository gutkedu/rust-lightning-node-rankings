use std::error::Error;

use serde::{Deserialize, Serialize};

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

pub fn fetch_node_rankings() -> Result<Vec<NodeConnectivityRanking>, Box<dyn Error>> {
    const URL: &str = "https://mempool.space/api/v1/lightning/nodes/rankings/connectivity";

    let http_client = ureq::Agent::new();

    let response = http_client.get(URL).call()?;

    let res: Vec<NodeConnectivityRanking> = serde_json::from_str(&response.into_string()?)?;

    if res.len() == 0 {
        return Err("No data found".into());
    }

    Ok(res)
}
