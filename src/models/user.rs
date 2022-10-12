use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub name: String,
    pub password: String,
    // type enum
    // key
    // file path
}

impl User {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            password: String::new(),
        }
    }
}
