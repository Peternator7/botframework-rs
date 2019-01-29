pub mod http;
pub mod middleware;

use bot_schema::Activity;
use reqwest::Client;

pub trait BotError: std::error::Error {
    fn status_code(&self) -> usize;
}

pub trait Bot
{
    type Error: BotError;
    fn handle_request(&self, context: &BotContext, activity: &Activity) -> Result<(), Self::Error>;
}

impl <F,E> Bot for F
    where F: Fn(&BotContext, &Activity) -> Result<(),E>,
          E: BotError
{
    type Error = E;
    fn handle_request(&self, context: &BotContext, activity: &Activity) -> Result<(), E> {
        self(context, activity)
    }
}

#[derive(Debug, Clone)]
pub struct BotContext {
    client: Client,
}

impl BotContext {
    pub fn new() -> BotContext {
        BotContext::default()
    }

    pub fn reply(&self, activity: &Activity) -> Result<(), Box<dyn std::error::Error>> {
        http::reply_to_activity(&self.client, activity)
    }
}

impl std::default::Default for BotContext {
    fn default() -> BotContext {
        BotContext {
            client: Client::new()
        }
    }
}