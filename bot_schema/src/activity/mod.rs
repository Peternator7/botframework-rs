
// Crate dependencies
use crate::channels::ChannelId;
use crate::descriptors::{ChannelAccount, ConversationAccount};
use crate::entity::Entity;
use crate::activity::message::MessageDetails;
// Std dependencies
// External Dependencies
use serde_derive::{Serialize, Deserialize};

pub mod message;
pub mod reaction;
pub mod update;

pub use reaction::MessageReaction;
pub use update::{ContactRelationUpdateDetails, ConversationUpdateDetails};

#[derive(Serialize,Deserialize, Clone, Debug, Default, PartialEq)]
#[serde(rename_all="camelCase")]
pub struct Activity {
    #[serde(default)]
    pub id: String,

    #[serde(default)]
    pub channel_id: ChannelId,

    #[serde(default)]
    pub from: ChannelAccount,

    #[serde(default)]
    pub recipient: ChannelAccount,

    #[serde(default)]
    pub conversation: ConversationAccount,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub timestamp: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub local_timestamp: Option<chrono::DateTime<chrono::FixedOffset>>,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub service_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub reply_to_id: Option<String>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub entities: Vec<Entity>,

    #[serde(flatten)]
    pub ty: ActivityType,
}

impl Activity {
    pub fn new() -> Activity {
        Activity::default()
    }

    fn respond_with_inner(&self) -> Activity {
        Activity {
            channel_id: self.channel_id,
            service_url: self.service_url.clone(),
            timestamp: Some(chrono::Utc::now()),
            conversation: self.conversation.clone(),
            from: self.recipient.clone(),
            recipient: self.from.clone(),
            reply_to_id: Some(self.id.clone()),
            ..Activity::default()
        }
    }

    pub fn respond_with_text(&self, text:String) -> Activity {
        let mut response = self.respond_with_inner();
        response.ty = ActivityType::Message(MessageDetails::with_text(text));
        response
    }

    pub fn respond_with_markdown(&self, text:String) -> Activity {
        let mut response = self.respond_with_inner();
        response.ty = ActivityType::Message(MessageDetails::with_markdown(text));
        response
    }

    pub fn respond_with_xml(&self, text:String) -> Activity {
        let mut response = self.respond_with_inner();
        response.ty = ActivityType::Message(MessageDetails::with_xml(text));
        response
    }

    pub fn respond_with_attachments(&self, attachments:Vec<crate::attachments::Attachment>) -> Activity {
        let mut response = self.respond_with_inner();
        response.ty = ActivityType::Message(MessageDetails::with_attachments(attachments));
        response
    }

    pub fn respond_with_typing(&self) -> Activity {
        let mut response = self.respond_with_inner();
        response.ty = ActivityType::Typing;
        response
    }
}

#[derive(Serialize,Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub enum ActivityType {
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

    #[serde(other)]
    Unknown
}

impl std::default::Default for ActivityType {
    fn default() -> ActivityType {
        ActivityType::Unknown
    }
}
