// src/utils/jwt_utils.rs

use jsonwebtoken::{encode, Header, Algorithm, EncodingKey};
use std::env;
use chrono;
use crate::models::user_jwt::UserJWT;


pub fn create_jwt(user_id: i32) -> Result<String, jsonwebtoken::errors::Error> {
    let private_key = env::var("PRIVATE_KEY").expect("PRIVATE_KEY must be set");
    let expiration = chrono::Utc::now() + chrono::Duration::days(1);
    let claims = UserJWT::new(
        user_id,
        expiration
    );
    let encoding_key = EncodingKey::from_rsa_pem(private_key.as_bytes())?;
    encode(&Header::new(Algorithm::RS256), &claims, &encoding_key)
}
