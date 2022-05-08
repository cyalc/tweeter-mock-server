use rocket::serde::{Serialize};

#[derive(Serialize)]
pub(crate) struct User {
    id: String,
    name: String,
    user_name: String,
}

impl User {
    pub async fn get(id: &str) -> User {
        User {
            id: id.to_string(),
            name: format!("Name {}", id),
            user_name: format!("@{}", id),
        }
    }
}