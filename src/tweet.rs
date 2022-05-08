use rocket::serde::{Serialize};
use crate::user::User;

#[derive(Serialize)]
pub(crate) struct Tweet {
    id: String,
    user: User,
    body: String,
    like_count: u64,
}

impl Tweet {
    pub async fn timeline() -> Vec<Tweet> {
        vec![
            Tweet {
                id: "4235".to_string(),
                user: User::get("1").await,
                body: "What a tweet?".to_string(),
                like_count: 45,
            },
            Tweet {
                id: "42335".to_string(),
                user: User::get("2").await,
                body: "What a tweet huh?".to_string(),
                like_count: 456,
            },
        ]
    }
}
