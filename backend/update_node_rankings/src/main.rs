mod mempool_api;
mod utils;

use lambda_runtime::{run, service_fn, tracing, Error, LambdaEvent};

use common::dtos::NewNodeDTO;
use common::dynamo::{create_node, new_dynamo_client};
use serde::Deserialize;

use crate::utils::{convert_satoshi_to_btc, convert_unix_to_iso};

#[derive(Deserialize)]
struct Request {}

#[tokio::main]
async fn main() -> Result<(), Error> {
    common::init_tracing();
    run(service_fn(function_handler)).await
}

async fn function_handler(_event: LambdaEvent<Request>) -> Result<(), Error> {
    // Fetch the data from the API
    let fetch_result = mempool_api::fetch_node_rankings();
    let node_rankings = match fetch_result {
        Ok(rankings) => rankings,
        Err(e) => {
            tracing::error!("Failed to fetch node rankings: {:?}", e);
            return Err(Error::from("Failed to fetch node rankings"));
        }
    };

    // Initialize the DynamoDB client
    let client = new_dynamo_client().await;
    const TABLE_NAME: &str = "NodesRankingTable";

    for node in node_rankings {
        let first_seen_iso = convert_unix_to_iso(node.firstSeen);

        let capacity_btc = convert_satoshi_to_btc(node.capacity);

        let new_node = NewNodeDTO {
            public_key: node.publicKey,
            alias: node.alias,
            channels: node.channels as u64,
            capacity_in_btc: capacity_btc.to_string(),
            first_seen_iso,
            updated_at: node.updatedAt,
        };

        create_node(&client, TABLE_NAME, new_node).await?;
    }

    Ok(())
}
