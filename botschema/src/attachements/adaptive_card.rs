use serde_derive::{Serialize, Deserialize};

fn schema_fn() -> &'static str {
    "http://adaptivecards.io/schemas/adaptive-card.json"
}

#[derive(Serialize,Deserialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AdaptiveCard {
    #[serde(rename = "$content", skip_deserializing, default = "schema_fn")]
    pub schema: &'static str,
}

