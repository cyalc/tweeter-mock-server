mod tweet;

#[macro_use]
extern crate rocket;

use rocket::serde::json::{Json};
use crate::tweet::Tweet;

#[get("/timeline")]
fn timeline() -> Json<Vec<Tweet>> {
    Json(
        vec![
            Tweet {
                id: "4235".to_string(),
                user_name: "cyalc".to_string(),
                body: "What a tweet?".to_string(),
                like_count: 45,
            },
            Tweet {
                id: "42335".to_string(),
                user_name: "elon".to_string(),
                body: "What a tweet huh?".to_string(),
                like_count: 456,
            }
        ]

    )
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
