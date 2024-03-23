// src/utils/jwt_utils.rs

use jsonwebtoken::{encode, Header, Algorithm, EncodingKey};
use std::env;
use chrono;
use crate::models::user_jwt::UserJWT;
use jsonwebtoken::{decode, DecodingKey, Validation, TokenData};


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

pub fn decode_jwt(token: &str) -> Result<TokenData<UserJWT>, jsonwebtoken::errors::Error> {
    let public_key = env::var("PUBLIC_KEY").expect("PUBLIC_KEY must be set");
    let decoding_key = DecodingKey::from_rsa_pem(public_key.as_bytes())?;
    let validation = Validation::new(Algorithm::RS256);
    decode::<UserJWT>(token, &decoding_key, &validation)
}
