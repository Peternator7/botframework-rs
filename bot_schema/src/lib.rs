#![allow(dead_code)]

pub mod attachments;
pub mod descriptors;
pub mod activity;
pub mod channels;
pub mod entity;

mod empty_str_visitor;

pub use crate::activity::{Activity, ActivityType};
pub use crate::activity::message::{MessageDetails, TextFormat};
pub use crate::attachments::Attachment;
pub use crate::attachments::hero_card::HeroCard;
pub use crate::attachments::actions::{CardAction, CardImage};
pub use crate::channels::ChannelId;
pub use crate::descriptors::*;

#[cfg(test)]
mod tests {
    use crate::*;

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
            timestamp: None,
            local_timestamp: None,
            // details: ActivityDetails::Message()
            ty: ActivityType::Message(MessageDetails {
                text_format: TextFormat::Markdown,
                text: Some("Hello World".into()),
                locale: Some("en-us".into()),
                attachments: vec![
                    HeroCard::builder()
                        .title("Peter")
                        .action(CardAction::ImBack {
                            title: Some("Im' a title".into()),
                            value: Some("Interesting fact".into()),
                        }).to_card().into()
                ],
            }),
        };

        println!("{}", serde_json::to_string(&a).unwrap());
    }

    #[test]
    fn test_2() {
        let json = r#"
            {
              "channelId": "emulator",
              "conversation": {
                "id": "afd7d110-188e-11e9-9acf-7b2777102201|livechat"
              },
              "from": {
                "id": "1",
                "name": "Bot",
                "role": "bot"
              },
              "id": "240666a0-188f-11e9-9e11-b74b4ead2f63",
              "inputHint": "acceptingInput",
              "localTimestamp": "2019-01-14T22:31:07-08:00",
              "locale": "en-us",
              "recipient": {
                "id": "7bcea511-19f0-4d6e-a4e7-186f1ae71a19",
                "role": "user"
              },
              "replyToId": "23feec90-188f-11e9-9e11-b74b4ead2f63",
              "serviceUrl": "http://localhost:52619",
              "text": "Type anything to see another card.",
              "timestamp": "2019-01-15T06:31:07.530Z",
              "type": "message"
            }"#;

        let actual = serde_json::from_str::<Activity>(json).unwrap();

        assert_eq!(crate::channels::ChannelId::Emulator, actual.channel_id);
        assert_eq!(Some("http://localhost:52619".to_string()), actual.service_url);
    }
}
