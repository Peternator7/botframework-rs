use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Entity {
    #[serde(rename = "type")]
    pub _type: String,
    #[serde(flatten)]
    pub data: std::collections::HashMap<String, serde_json::Value>,
}