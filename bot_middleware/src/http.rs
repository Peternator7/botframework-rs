use reqwest::{Client, Url};
use bot_schema::Activity;
use percent_encoding::{utf8_percent_encode, USERINFO_ENCODE_SET};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum BotResponseError {
    MissingServiceUrlError,
    PostToConnectorError,
}

impl std::fmt::Display for BotResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            BotResponseError::MissingServiceUrlError => write!(f, "Service Url is missing"),
            BotResponseError::PostToConnectorError => write!(f, "Connector Response does not indicate success"),
        }
    }
}

impl std::error::Error for BotResponseError {}

fn reply_inner(client:&Client, url:Url, activity: &Activity) -> Result<(), Box<dyn std::error::Error>> {
    let resp = client.post(url)
        .json(activity)
        .send()?;

    if resp.status().is_success() {
        Ok(())
    } else {
        Err(BotResponseError::PostToConnectorError)?
    }
}

pub fn reply_to_activity(client: &Client, activity: &Activity) -> Result<(), Box<dyn std::error::Error>> {
    let service_url = activity.service_url.as_ref().ok_or(BotResponseError::MissingServiceUrlError)?;

    let url = format!("{}/v3/conversations/{}/activities/{}",
        service_url,
        utf8_percent_encode(&activity.conversation.id, USERINFO_ENCODE_SET),
        utf8_percent_encode(&activity.id, USERINFO_ENCODE_SET));

    let url = Url::parse(&*url)?;
    reply_inner(client, url, activity)
}

pub fn resume_conversation(client: &Client, activity: &Activity) -> Result<(), Box<dyn std::error::Error>> {
    let service_url = activity.service_url.as_ref().ok_or(BotResponseError::MissingServiceUrlError)?;

    let url = format!("{}/v3/conversations/{}/activities",
        service_url,
        utf8_percent_encode(&activity.conversation.id, USERINFO_ENCODE_SET));

    let url = Url::parse(&*url)?;
    reply_inner(client, url, activity)
}

pub fn start_conversation(client: &Client, activity: &Activity) -> Result<(), Box<dyn std::error::Error>> {
    let service_url = activity.service_url.as_ref().ok_or(BotResponseError::MissingServiceUrlError)?;

    let url = format!("{}/v3/conversations", service_url);

    let url = Url::parse(&*url)?;
    reply_inner(client, url, activity)
}