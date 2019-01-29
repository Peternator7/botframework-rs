//! Entities are small pieces of additional information provided by the botframework connector.
//! They are channel specific. These can be read from the connector, but setting them typically
//! won't have any effect on the response.
use serde_derive::{Serialize, Deserialize};

/// Entities are small pieces of additional information provided by the botframework connector.
/// They are channel specific. These can be read from the connector, but setting them typically
/// won't have any effect on the response.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Entity {
    /// They "type" of the entity.
    #[serde(rename = "type")]
    pub ty: String,
    /// The additional fields on the entity. It is unstructured data provided by the channel.
    #[serde(flatten)]
    pub data: std::collections::HashMap<String, serde_json::Value>,
}