use serde_derive::{Serialize,Deserialize};

#[derive(Serialize,Deserialize, Clone, Copy, Eq, PartialEq, Debug)]
pub enum ChannelId {
    Cortana,
    DirectLine,
    Email,
    Facebook,
    GroupMe,
    Kik,
    MicrosoftTeams,
    Slack,
    Skype,
    SkypeForBusiness,
    Telegram,
    Twilio,
    WebChat,
    Emulator,
    #[serde(other)]
    Unknown,
}

impl std::default::Default for ChannelId {
    fn default() -> Self {
        ChannelId::Unknown
    }
}