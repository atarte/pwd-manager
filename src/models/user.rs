use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

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

#[derive(Debug, Clone, Hash, Deserialize, Serialize)]
pub struct User {
    pub name: String,
    pub password: u64,
    // type enum
    // key
    // file path
}

impl Init for User {
    fn new() -> Self {
        Self {
            name: String::new(),
            password: 0u64,
        }
    }
}

impl User {
    pub fn hash_password(&mut self, password_string: String) {
        let mut hash = DefaultHasher::new();
        password_string.hash(&mut hash);
        self.password = hash.finish();
    }
}
