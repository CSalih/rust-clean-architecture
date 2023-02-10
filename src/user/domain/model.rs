use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub status: String,
}

impl User {
    pub fn new(username: String, status: String) -> Self {
        User { username, status }
    }
}
