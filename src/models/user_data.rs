use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct UserData {

}

impl UserData {
    pub fn new() -> Self {
        Self{

        }
    }
}
