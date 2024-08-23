use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};

use common::dynamo::{find_many_nodes, new_dynamo_client};

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
    common::init_tracing();
    run(service_fn(function_handler)).await
}

async fn function_handler(_event: LambdaEvent<Request>) -> Result<Response, Error> {
    let client = new_dynamo_client().await;
    const TABLE_NAME: &str = "NodesRankingTable";

    let items = find_many_nodes(&client, TABLE_NAME).await;

    let resp = Response {
        statusCode: 200,
        body: items?,
    };
    Ok(resp)
}
