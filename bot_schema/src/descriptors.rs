//! Descriptors are small pieces of information that describe the conversation or members of the conversation.
//! The botframework connector will populate many of these fields when it sends a message request, and they
//! provide useful information about who is talking and how to address said individual. Many of these fields
//! are also channel-specific with regards to how they are formatted. Some channels may use email as an
//! identifier while others intentionally use an identifier that has no meaning outside of the conversation.
//!
//! If you desire to send pro-active notifications to a user, you should save the `ChannelAccount` and
//! `ConversationAccount` to storage so that the conversation can be resumed later. Also be aware that
//! many channels will not let your interact with a user who has not previously interacted with your
//! bot.
use serde_derive::{Serialize, Deserialize};

/// Contains useful information pertaining to a member of the conversation. The `id` field is the
/// primary identifier in a `ChannelAccount`. A user's name and role are not guarenteed to be
/// unique. Some fields on the Channel Account are set by the botframework connector on requests,
/// but not necessary to set in your response. These fields are labeled *Read-Only* in the documenation.
/// Setting them will not cause any errors, but they will not affect the behavior the end-customer sees.
#[derive(Serialize,Deserialize,Eq, PartialEq, Hash, Clone, Default, Debug)]
pub struct ChannelAccount {
    /// The unique identifer provided to a user or account by the channel. This may be an
    /// email address or a string of unique characters. It is channel dependent.
    #[serde(default)]
    pub id: String,

    /// *Read-Only* A friendly name for the account. This is typically the name you should
    /// address the client by when your bot is holding a conversation.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub name: Option<String>,

    /// *Read-Only* The role this account has in the conversation.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub role: Option<String>,
}

impl ChannelAccount {
    /// Create a new ChannelAccount. The only field that needs to be set on the
    /// ChannelAccount is the `id` so this is the preferred constructor. The `id`
    /// is typically generated by the channel itself (i.e. facebook) and then passed to your bot.
    /// The actual structure of id is channel dependent as well. Some channels use a unique identifier
    /// while others use the client's email address.
    pub fn new(id: String) -> ChannelAccount
    {
        ChannelAccount {
            id: id.into(),
            ..ChannelAccount::default()
        }
    }
}

/// Capture information about the conversation this message was coming from. Conversation Account
/// includes the unique identifier for the conversation as well as it's name if it has one, and
/// information if it is a group chat. Some of the fields on this struct are set by the
/// botframework connector and can be read by your bot, but they will not have an effect if you
/// set it yourself. These fields are indicated by a "*Read-Only*" at the beginning of the field.
/// It is safe to set these fields, but they will have no effect when sent to the connector.
#[derive(Serialize,Deserialize,Eq, PartialEq, Hash, Clone, Default, Debug)]
#[serde(rename_all="camelCase")]
pub struct ConversationAccount {
    /// The Conversation ID. You typically will not generate this yourself, but
    /// instead re-use the ID given to you by the botframework connector. This can be used
    /// to respond to a message immediately or it can be saved and used later to create a
    /// "proactive" message.
    #[serde(default)]
    pub id: String,

    /// *Read-Only* The name of the conversation. Not every conversation has a name, but
    /// some channels support renaming a chat.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub name: Option<String>,

    /// *Read-Only* Indicates whether the conversation is a group message. `conversation_type`
    /// provides more details about the type of conversation this is.
    #[serde(default)]
    pub is_group: bool,

    /// *Read-Only* The conversation type. This is a more detailed description of the type of
    /// group chat.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub conversation_type: Option<String>,
}

impl ConversationAccount {
    /// Create a new `ConversationAccount` given a Conversation id. This is useful for resuming
    /// a conversation with an individual.
    pub fn new(id: String) -> ConversationAccount {
        ConversationAccount {
            id: id,
            ..ConversationAccount::default()
        }
    }
}