use self::hero_card::HeroCard;
use self::adaptive_card::AdaptiveCard;

use std::convert::From;

use serde_derive::{Serialize, Deserialize};

pub mod actions;
pub mod adaptive_card;
pub mod hero_card;

// #[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
// #[serde(rename_all = "camelCase")]
// pub enum ContentType {
//     ContentUrl(String),
//     Content(Content),
// }

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(tag = "contentType")]
#[serde(rename_all = "camelCase")]
pub enum Attachment {
    /// Represents content that is attached to a
    ContentUrl { content_url:String },
    /// Hero cards.
    #[serde(rename = "application/vnd.microsoft.card.hero")]
    HeroCard { content: self::hero_card::HeroCard },
    /// Adaptive Cards.
    #[serde(rename = "application/vnd.microsoft.card.adaptive")]
    AdaptiveCard { content: self::adaptive_card::AdaptiveCard },
    Other(serde_json::Value),
}

impl From<HeroCard> for Attachment {
    fn from(card: HeroCard) -> Attachment {
        Attachment::HeroCard {
            content: card
        }
    }
}

impl From<AdaptiveCard> for Attachment {
    fn from(card: AdaptiveCard) -> Attachment {
        Attachment::AdaptiveCard {
            content: card
        }
    }
}

// #[derive(Serialize,Deserialize, Clone, PartialEq, Debug)]
// #[serde(rename_all = "camelCase")]
// pub struct Attachment {
//     pub content_type: String,
//     // #[serde(skip_serializing_if = "Option::is_none", default)]
//     // pub name: Option<String>,
//     #[serde(flatten)]
//     pub content: Content
// }