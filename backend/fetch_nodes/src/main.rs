use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};

use common::dynamo::{find_many_nodes, new_dynamo_client};
use common::tracing::init_tracing;

#[derive(Deserialize)]
struct Request {}

#[derive(Serialize)]
#[allow(non_snake_case)]
struct Response {
    statusCode: i32,
    body: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    init_tracing();
    run(service_fn(fetch_nodes_handler)).await
}

// Handler function that is activated by a ApiGatewayProxyRequest
async fn fetch_nodes_handler(_event: LambdaEvent<Request>) -> Result<Response, Error> {
    // Initialize the DynamoDB client
    let client = new_dynamo_client().await;
    const TABLE_NAME: &str = "NodesRankingTable";

    let items = find_many_nodes(&client, TABLE_NAME).await;

    let resp = Response {
        statusCode: 200,
        body: items?,
    };
    Ok(resp)
}
