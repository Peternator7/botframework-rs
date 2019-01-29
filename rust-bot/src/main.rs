use actix_web::{server, http, Json, App, HttpRequest, Responder, FromRequest, AsyncResponder};
use bot_schema::{Activity, ActivityType};
use futures::future::Future;
use reqwest::{Client, Url};
use percent_encoding::{utf8_percent_encode, USERINFO_ENCODE_SET};

fn greet(req: &HttpRequest) -> impl Responder {
    Json::<Activity>::extract(req)
        .from_err::<actix_web::Error>()
        .map(|val: Json<Activity>| {  // <- deserialized value
            match val.ty {
                ActivityType::Message(ref details) => {
                    let response = val.respond_with_text(
                        format!("{}.......zzzzzzz", details.text.as_ref().unwrap())
                    );

                    let client = Client::new();
                    let url = format!("{}/v3/conversations/{}/activities/{}",
                        val.service_url.as_ref().unwrap(),
                        utf8_percent_encode(&val.conversation.id, USERINFO_ENCODE_SET),
                        utf8_percent_encode(&val.id, USERINFO_ENCODE_SET));

                    client.post(Url::parse(&*url).expect("Failed to parse url"))
                        .json(&response)
                        .send()
                        .unwrap();
                },
                _ => {

                }
            }

            Json(())
        })
       .responder()
}

fn main() {
    let t = vec!["Peter", "is", "cool"];

    println!("Starting Server");
    server::new(|| {
        println!("Creating app");
        App::new()
            .resource("/api/messages", |r| r.method(http::Method::POST).f(greet))
            // .resource("/", |r| r.f(greet))
            // .resource("/{name}", |r| r.f(greet))
    })
    .bind("127.0.0.1:8000")
    .expect("Can not bind to port 8000")
    .run();
}
