use aws_sdk_dynamodb::{types::AttributeValue, Client};
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};

#[derive(Deserialize)]
struct Request {}

#[derive(Serialize)]
struct Response {
    statusCode: i32,
    body: String,
}

// Helper function to convert AttributeValue to a serializable type
fn attribute_value_to_json(value: &AttributeValue) -> serde_json::Value {
    match value {
        AttributeValue::S(s) => json!(s),
        AttributeValue::N(n) => json!(n),
        AttributeValue::B(b) => json!(b.as_ref()),
        AttributeValue::Bool(b) => json!(b),
        AttributeValue::Null(_) => json!(null),
        AttributeValue::M(m) => {
            let map: serde_json::Map<String, serde_json::Value> = m
                .iter()
                .map(|(k, v)| (k.clone(), attribute_value_to_json(v)))
                .collect();
            json!(map)
        }
        AttributeValue::L(l) => {
            let list: Vec<serde_json::Value> =
                l.iter().map(|v| attribute_value_to_json(v)).collect();
            json!(list)
        }
        _ => json!(null),
    }
}

async fn function_handler(_event: LambdaEvent<Request>) -> Result<Response, Error> {
    // Initialize the DynamoDB client
    let shared_config = aws_config::load_from_env().await;
    let client = Client::new(&shared_config);
    const TABLE_NAME: &str = "NodesRankingTable";

    let query = client
        .query()
        .table_name(TABLE_NAME)
        .index_name("LSI1")
        .scan_index_forward(false)
        .key_condition_expression("#PK = :PK")
        .expression_attribute_names("#PK", "PK")
        .expression_attribute_values(":PK", AttributeValue::S("NODE".to_string()))
        .send()
        .await?;

    // Extract and format items
    let items: Vec<_> = query
        .items
        .unwrap_or_default()
        .into_iter()
        .map(|item| {
            let mut map = Map::new();
            map.insert(
                "public_key".to_string(),
                attribute_value_to_json(
                    item.get("public_key")
                        .unwrap_or(&AttributeValue::S("".to_string())),
                ),
            );
            map.insert(
                "alias".to_string(),
                attribute_value_to_json(
                    item.get("alias")
                        .unwrap_or(&AttributeValue::S("".to_string())),
                ),
            );
            map.insert(
                "capacity".to_string(),
                attribute_value_to_json(
                    item.get("capacity")
                        .unwrap_or(&AttributeValue::S("0".to_string())),
                ),
            );
            map.insert(
                "first_seen".to_string(),
                attribute_value_to_json(
                    item.get("first_seen")
                        .unwrap_or(&AttributeValue::S("".to_string())),
                ),
            );
            Value::Object(map)
        })
        .collect();

    // Prepare the response
    let resp = Response {
        statusCode: 200,
        body: json!(items).to_string(),
    };

    // Return `Response` (it will be serialized to JSON automatically by the runtime)
    Ok(resp)
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
