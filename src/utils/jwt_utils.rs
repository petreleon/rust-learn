// src/utils/jwt_utils.rs

use chrono::{self, Duration};
use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use std::env;

use crate::models::user_jwt::UserJWT;

const DEFAULT_JWT_EXPIRATION_SECONDS: i64 = 24 * 60 * 60;

fn jwt_expiration_seconds_from_env_value(value: Option<&str>) -> i64 {
    value
        .and_then(|raw| raw.trim().parse::<i64>().ok())
        .filter(|seconds| *seconds > 0)
        .unwrap_or(DEFAULT_JWT_EXPIRATION_SECONDS)
}

fn jwt_expiration_seconds() -> i64 {
    jwt_expiration_seconds_from_env_value(env::var("JWT_EXPIRATION_SECONDS").ok().as_deref())
}

pub fn create_jwt(user_id: i32) -> Result<String, jsonwebtoken::errors::Error> {
    let private_key = env::var("PRIVATE_KEY").expect("PRIVATE_KEY must be set");
    let expiration = chrono::Utc::now() + Duration::seconds(jwt_expiration_seconds());
    let claims = UserJWT::new(user_id, expiration);
    let encoding_key = EncodingKey::from_rsa_pem(private_key.as_bytes())?;
    encode(&Header::new(Algorithm::RS256), &claims, &encoding_key)
}

pub fn decode_jwt(token: &str) -> Result<TokenData<UserJWT>, jsonwebtoken::errors::Error> {
    let public_key = env::var("PUBLIC_KEY").expect("PUBLIC_KEY must be set");
    let decoding_key = DecodingKey::from_rsa_pem(public_key.as_bytes())?;
    let validation = Validation::new(Algorithm::RS256);
    decode::<UserJWT>(token, &decoding_key, &validation)
}

#[cfg(test)]
mod tests {
    use super::{jwt_expiration_seconds_from_env_value, DEFAULT_JWT_EXPIRATION_SECONDS};

    #[test]
    fn uses_default_jwt_expiration_when_unset() {
        assert_eq!(
            jwt_expiration_seconds_from_env_value(None),
            DEFAULT_JWT_EXPIRATION_SECONDS
        );
    }

    #[test]
    fn parses_positive_jwt_expiration() {
        assert_eq!(jwt_expiration_seconds_from_env_value(Some("3600")), 3600);
    }

    #[test]
    fn falls_back_for_invalid_jwt_expiration() {
        assert_eq!(
            jwt_expiration_seconds_from_env_value(Some("not-a-number")),
            DEFAULT_JWT_EXPIRATION_SECONDS
        );
        assert_eq!(
            jwt_expiration_seconds_from_env_value(Some("0")),
            DEFAULT_JWT_EXPIRATION_SECONDS
        );
        assert_eq!(
            jwt_expiration_seconds_from_env_value(Some("-60")),
            DEFAULT_JWT_EXPIRATION_SECONDS
        );
    }
}
