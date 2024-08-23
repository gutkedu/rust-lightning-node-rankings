use std::collections::HashMap;

use aws_sdk_dynamodb::{types::AttributeValue, Client, Error};
use serde_json::{json, Map, Value};

use crate::{dtos::NewNodeDTO, dynamo_utils::attribute_value_to_json};

pub async fn new_dynamo_client() -> Client {
    let shared_config = aws_config::load_from_env().await;
    let client = Client::new(&shared_config);
    client
}

pub async fn create_node(
    client: &Client,
    table_name: &str,
    new_node: NewNodeDTO,
) -> Result<(), Error> {
    // Create an item for the table
    let item = HashMap::from([
        ("PK".to_string(), AttributeValue::S("NODE".to_string())),
        (
            "SK".to_string(),
            AttributeValue::S(format!("PUBLIC_KEY#{}", new_node.public_key)),
        ),
        (
            "LSI1".to_string(),
            AttributeValue::N(new_node.channels.to_string()),
        ),
        (
            "channels".to_string(),
            AttributeValue::N(new_node.channels.to_string()),
        ),
        (
            "updated_at".to_string(),
            AttributeValue::N(new_node.updated_at.to_string()),
        ),
        (
            "public_key".to_string(),
            AttributeValue::S(new_node.public_key.clone()),
        ),
        (
            "alias".to_string(),
            AttributeValue::S(new_node.alias.clone()),
        ),
        (
            "capacity".to_string(),
            AttributeValue::S(new_node.capacity_in_btc.to_string()),
        ),
        (
            "first_seen".to_string(),
            AttributeValue::S(new_node.first_seen_iso.to_string()),
        ),
    ]);

    // Put the item into the table
    client
        .put_item()
        .table_name(table_name)
        .set_item(Some(item))
        .send()
        .await?;

    Ok(())
}

pub async fn find_many_nodes(client: &Client, table_name: &str) -> Result<String, Error> {
    let query = client
        .query()
        .table_name(table_name)
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

    // Serialize to JSON string
    let json = json!(items).to_string();

    // Return the JSON string
    Ok(json)
}
