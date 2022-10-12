use serde::{Deserialize, Serialize};

use super::Init;

#[derive(Debug, Deserialize, Serialize)]
pub struct UsersList {
    pub users: Vec<User>,
}

impl Init for UsersList {
    fn new() -> Self {
        Self {
            users: Vec::new(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub name: String,
    pub password: String,
    // type enum
    // key
    // file path
}

impl Init for User {
    fn new() -> Self {
        Self {
            name: String::new(),
            password: String::new(),
        }
    }
}

impl Copy for User {
    
}

impl Clone for User {
    fn clone(&self) -> Self {
        *self
    }
}
