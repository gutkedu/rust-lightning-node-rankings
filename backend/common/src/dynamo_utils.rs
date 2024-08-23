use aws_sdk_dynamodb::types::AttributeValue;
use serde_json::json;

/// Convert an `AttributeValue` to a `serde_json::Value`.
/// - The function returns a `serde_json::Value` that represents the `AttributeValue`.
/// - The `AttributeValue` can be of type `S`, `N`, `B`, `Bool`, `Null`, `M`, or `L`.
pub fn attribute_value_to_json(value: &AttributeValue) -> serde_json::Value {
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
