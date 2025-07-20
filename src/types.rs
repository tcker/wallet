use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct AptosResource {
    #[serde(rename = "type")]
    pub resource_type: String,
    pub data: Value,
}
