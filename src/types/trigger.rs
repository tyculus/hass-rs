use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct HassTrigger {
    pub platform: String,
    pub domain: String,
    pub device_id: String,
    #[serde(rename = "type")]
    pub trigger_type: String,
    pub subtype: Option<String>,
    pub metadata: HashMap<String, Value>,
}
