mod tweet;
mod user;

#[macro_use]
extern crate rocket;

use rocket::serde::json::{Json};
use crate::tweet::Tweet;

#[get("/timeline")]
async fn timeline() -> Json<Vec<Tweet>> {
    let timeline  = Tweet::timeline().await;
    Json(timeline)
}

#[get("/")]
fn index() -> &'static str {
    "Tweeter mock server"
}

#[rocket::main]
async fn main() {
    if let Err(e) = rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![timeline])
        .launch()
        .await {
        println!("Error when launching rocket.");
        drop(e);
    };
}
