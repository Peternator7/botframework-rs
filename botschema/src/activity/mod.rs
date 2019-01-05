
// Crate dependencies
use crate::channels::ChannelId;
use crate::descriptors::{ChannelAccount, ConversationAccount};
use crate::entity::Entity;
use crate::activity::message::MessageDetails;
// Std dependencies
// External Dependencies
use serde_derive::{Serialize, Deserialize};

pub mod message;

#[derive(Serialize,Deserialize, Debug, PartialEq)]
#[serde(rename_all="camelCase")]
pub struct Activity {
    pub channel_id: ChannelId,
    pub from: ChannelAccount,
    pub recipient: ChannelAccount,
    pub conversation: ConversationAccount,

    #[serde(default)]
    pub timestamp: String,
    #[serde(default)]
    pub local_timestamp: String,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub service_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub reply_to_id: Option<String>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub entities: Vec<Entity>,

    #[serde(flatten)]
    pub details: ActivityDetails,
}

#[derive(Serialize,Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub enum ActivityDetails {
    Message (MessageDetails),
    ContactRelationUpdate (ContactRelationUpdateDetails),
    ConversationUpdate(ConversationUpdateDetails),
    DeleteUserData {

    },
    EndOfConversation {
        code: String,
        text: String,
    },
    Event {

    },
    InstallationUpdate (ContactRelationUpdateDetails),
    Invoke {

    },
    MessageReaction {
        #[serde(default)]
        reactions_added: Vec<MessageReaction>,
        #[serde(default)]
        reactions_removed: Vec<MessageReaction>,
    },
    Typing,
    MessageUpdate {
        #[serde(flatten)]
        remaining: std::collections::HashMap<String,serde_json::Value>,
    },
    MessageDelete,
    Suggestion {

    },
    Trace {
        #[serde(skip_serializing_if = "Option::is_none", default)]
        name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", default)]
        label: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", default)]
        relates_to: Option<()>,
        #[serde(skip_serializing_if = "Option::is_none", default)]
        value_type: Option<String>,
        value: serde_json::Value,
    },
    Handoff,
}


#[derive(Serialize,Deserialize, Clone, Eq, PartialEq, Hash, Default, Debug)]
pub struct MessageReaction {
    #[serde(rename = "type")]
    pub _type: String,
}

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