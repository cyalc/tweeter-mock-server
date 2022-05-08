use rocket::serde::{Serialize};
use crate::user::User;

#[derive(Serialize)]
pub(crate) struct Tweet {
    pub id: String,
    pub user: User,
    pub body: String,
    pub like_count: u64,
}

impl Tweet {
    pub async fn timeline() -> Vec<Tweet> {
        vec![
            Tweet {
                id: "4235".to_string(),
                user: User {
                    id: "123".to_string(),
                    name: "Cagri Yalcinsoy".to_string(),
                    user_name: "cyalc".to_string(),
                },
                body: "What a tweet?".to_string(),
                like_count: 45,
            },
            Tweet {
                id: "42335".to_string(),
                user: User {
                    id: "1".to_string(),
                    name: "Elon Musk".to_string(),
                    user_name: "elon".to_string()
                },
                body: "What a tweet huh?".to_string(),
                like_count: 456,
            },
        ]
    }
}
