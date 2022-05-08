use rocket::serde::{Serialize};

#[derive(Serialize)]
pub(crate) struct Tweet {
    pub id: String,
    pub user_name: String,
    pub body: String,
    pub like_count: u64,
}