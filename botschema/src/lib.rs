#![allow(dead_code)]

pub mod attachements;
pub mod descriptors;
pub mod activity;
pub mod channels;
pub mod entity;

mod empty_str_visitor;

pub use crate::activity::{Activity, ActivityDetails};
pub use crate::activity::message::{MessageDetails, TextFormat};
pub use crate::attachements::Attachment;
pub use crate::attachements::hero_card::HeroCard;
pub use crate::attachements::actions::{CardAction, CardImage};
pub use crate::channels::ChannelId;
pub use crate::descriptors::*;

#[cfg(test)]
mod tests {
    use crate::*;

    use serde_json::json;

    #[test]
    fn it_works() {
        let a = Activity {
            channel_id: ChannelId::Facebook,
            from: ChannelAccount::default(),
            recipient: ChannelAccount::default(),
            conversation: ConversationAccount::default(),
            service_url: None,
            entities: vec![],
            reply_to_id: None,
            timestamp: String::new(),
            local_timestamp: String::new(),
            // details: ActivityDetails::Message()
            details: ActivityDetails::Message(MessageDetails {
                text_format: TextFormat::Markdown,
                text: Some("Hello World".into()),
                locale: Some("en-us".into()),
                attachments: vec![
                    HeroCard::builder()
                        .title("Peter")
                        .push_action(CardAction::ImBack {
                            title: Some("Im' a title".into()),
                            value: Some("Interesting fact".into()),
                        }).to_card().into()
                ],
            }),
        };

        println!("{}", serde_json::to_string(&a).unwrap());
    }
}
