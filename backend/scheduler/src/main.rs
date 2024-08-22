use std::collections::HashMap;

use chrono::{DateTime, NaiveDateTime, Utc};

// ...

use lambda_runtime::{run, service_fn, Error, LambdaEvent};

use serde::{Deserialize, Serialize};
use ureq::Agent;

use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::Client;

#[derive(Deserialize, Serialize)]
#[allow(non_snake_case)]
struct Node {
    publicKey: String,
    alias: String,
    channels: u32,
    capacity: u64,
    firstSeen: u64,
    updatedAt: u64,
}

#[derive(Deserialize)]
struct Request {}

async fn function_handler(_event: LambdaEvent<Request>) -> Result<(), Error> {
    // Fetch the data from the API
    let agent = Agent::new();
    let url = "https://mempool.space/api/v1/lightning/nodes/rankings/connectivity";
    let response = agent.get(url).call()?;
    let res: Vec<Node> = serde_json::from_str(&response.into_string()?)?;

    // Initialize the DynamoDB client
    let shared_config = aws_config::load_from_env().await;
    let client = Client::new(&shared_config);
    const TABLE_NAME: &str = "NodesRankingTable";

    // Loop through each Node object and put it into the table
    for node in res {
        // Convert Unix timestamp to ISO 8601 format
        let first_seen_iso =
            if let Some(datetime) = DateTime::<Utc>::from_timestamp(node.firstSeen as i64, 0) {
                datetime.to_rfc3339()
            } else {
                // Handle the case when `datetime` is `None`
                String::new() // Or any other default value or error handling logic
            };

        // Convert capacity from satoshis to bitcoins
        let capacity_btc = node.capacity as f64 / 100_000_000.0;

        let item = HashMap::from([
            ("pk".to_string(), AttributeValue::S("NODE".to_string())),
            (
                "sk".to_string(),
                AttributeValue::S(format!("PUBLIC_KEY#{}", node.publicKey)),
            ),
            (
                "public_key".to_string(),
                AttributeValue::S(node.publicKey.clone()),
            ),
            ("alias".to_string(), AttributeValue::S(node.alias.clone())),
            (
                "capacity".to_string(),
                AttributeValue::S(capacity_btc.to_string()),
            ),
            ("first_seen".to_string(), AttributeValue::S(first_seen_iso)),
        ]);

        // Put the item into the table
        let put_request = client
            .put_item()
            .table_name(TABLE_NAME)
            .set_item(Some(item));
        let put_response = put_request.send().await?;
        println!("Put Item Response: {:?}", put_response);
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
