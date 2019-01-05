#![feature(async_await)]
#![feature(futures_api)]
#![feature(await_macro)]

#[macro_use]
extern crate nom;

use actix_web::{server, App, HttpRequest, Responder};

mod additional;
pub mod tokens;

fn greet(req: &HttpRequest) -> impl Responder {
    let to = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", to)
}

fn main() {
    server::new(|| {
        App::new()
            .resource("/", |r| r.f(greet))
            .resource("/{name}", |r| r.f(greet))
    })
    .bind("127.0.0.1:8000")
    .expect("Can not bind to port 8000")
    .run();
}