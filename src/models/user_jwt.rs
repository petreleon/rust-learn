// models/user_jwt.rs

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct UserJWT {
    pub user_id: String, 
    pub exp: usize,
}
