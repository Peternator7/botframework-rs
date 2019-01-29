//! Channels are a core part of the Microsoft botframework. A channel is a platform for communicating
//! with a client such as Skype or Facebook. One of the primary goals of the botframework is to provide a
//! unified interface for interacting with a variety of different platforms.
//!
//! Some channels have specific functionality that they can implement. See
//! [this page](https://docs.microsoft.com/en-us/azure/bot-service/bot-builder-channeldata?view=azure-bot-service-4.0)
//! for more information about certain channels.
use serde_derive::{Serialize,Deserialize};

/// The name of a specific channel. Every message should have a `channel_id` set indicating what
/// platform the user is interacting with your bot through.
#[derive(Serialize,Deserialize, Clone, Copy, Eq, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ChannelId {
    Cortana,
    DirectLine,
    Email,
    Facebook,
    GroupMe,
    Kik,
    #[serde(rename = "msteams")]
    MicrosoftTeams,
    Slack,
    Skype,
    Telegram,
    Twilio,
    WebChat,
    Emulator,
    Sms,
    #[serde(other)]
    Unknown,
}

impl std::default::Default for ChannelId {
    fn default() -> Self {
        ChannelId::Unknown
    }
}