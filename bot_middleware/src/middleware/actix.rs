use actix_web::{Responder, App, HttpRequest, Json, FromRequest, AsyncResponder};
use bot_schema::Activity;
use futures::future::Future;
use reqwest::Client;
use std::sync::Arc;

pub trait BotMiddleWare<B> {
    fn add_bot(self, path: &str, bot: B) -> Self;
}

impl <B,S> BotMiddleWare<B> for App<S>
    where S: 'static,
          B: crate::Bot + 'static
{
    fn add_bot(self, path: &str, bot: B) -> Self
    {
        let bot = Arc::new(bot);
        self.resource(path, |r| {
            r.method(actix_web::http::Method::POST)
                .f(move |req| handler(req, bot.clone()))
        })
    }
}

fn handler<B,S: 'static>(req:&HttpRequest<S>, bot: Arc<B>) -> impl Responder
    where B: crate::Bot + 'static
{
    Json::<Activity>::extract(req)
        .from_err::<actix_web::Error>()
        .and_then(|val: Json<Activity>| {
            futures::future::ok::<Json<Activity>, actix_web::Error>(val)
        })
        .map(move |val: Json<Activity>| {
            let context = crate::BotContext {
                client: Client::new(),
            };

            bot.handle_request(&context, &val);
            val
        })
        .responder()
}