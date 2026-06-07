// src/models/user_jwt.rs

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct UserJWT {
    pub user_id: i32,
    pub exp: usize,
}

impl UserJWT {
    // Method to create a new UserJWT instance
    pub fn new(user_id: i32, exp: DateTime<Utc>) -> Self {
        UserJWT {
            user_id,
            exp: exp.timestamp() as usize,
        }
    }
}
