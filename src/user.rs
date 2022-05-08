use rocket::serde::{Serialize};

#[derive(Serialize)]
pub(crate) struct User {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) user_name: String,
}