use serde_derive::{Serialize, Deserialize};
use crate::props;

#[derive(Serialize,Deserialize, Clone, PartialEq, Default, Debug)]
pub struct InputNumber {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<serde_json::Number>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min: Option<serde_json::Number>,

    #[serde(default, skip_serializing_if="Option::is_none")]
    pub placeholder: Option<String>,

    #[serde(default, skip_serializing_if="Option::is_none")]
    pub value: Option<String>,

    #[serde(default)]
    pub id: String,

    #[serde(skip_serializing_if = "props::Spacing::is_default", default)]
    pub spacing: props::Spacing,

    #[serde(default)]
    pub seperator: bool,
}