use serde_derive::{Serialize, Deserialize};

#[derive(Serialize,Deserialize,Eq, PartialEq, Hash, Clone, Default, Debug)]
pub struct ChannelAccount {
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub role: Option<String>,
}

#[derive(Serialize,Deserialize,Eq, PartialEq, Hash, Clone, Default, Debug)]
#[serde(rename_all="camelCase")]
pub struct ConversationAccount {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub name: Option<String>,
    #[serde(default)]
    pub is_group: bool,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub conversation_type: Option<String>,
}