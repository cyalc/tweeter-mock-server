use rocket::serde::{Serialize};

#[derive(Serialize)]
pub(crate) struct User {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) user_name: String,
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