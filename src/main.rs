#[macro_use]
extern crate rocket;

#[get("/timeline")]
fn timeline() -> &'static str {
    "Hello, timeline!"
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
