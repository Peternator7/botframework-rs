use crate::descriptors::ConversationAccount;
use serde_derive::{Serialize, Deserialize};

#[derive(Serialize,Deserialize, Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum ContactRelationUpdateDetails {
    #[serde(rename = "add")]
    Added,
    #[serde(rename = "remove")]
    Removed,
}

#[derive(Serialize,Deserialize, Clone, Eq, PartialEq, Default, Debug, Hash)]
#[serde(rename_all = "camelCase")]
pub struct ConversationUpdateDetails {
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub members_added: Vec<ConversationAccount>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub members_removed: Vec<ConversationAccount>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub topic_name: Option<String>,
}