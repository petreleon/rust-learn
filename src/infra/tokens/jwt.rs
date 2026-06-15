use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use chrono::{self, Duration};
use futures::future::{BoxFuture, FutureExt};
use jsonwebtoken::{
    decode, encode,
    errors::{Error as JwtError, ErrorKind},
    Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use openssl::pkey::PKey;
use std::env;

use crate::application::identity::jwks::{JsonWebKeyOutput, JwksError, JwksOutput, JwksUseCase};
use crate::domain::identity::UserJWT;

const DEFAULT_JWT_EXPIRATION_SECONDS: i64 = 24 * 60 * 60;
const DEFAULT_JWT_KEY_ID: &str = "rust-learn-local";

#[derive(Clone)]
pub struct EnvJwksUseCase;

impl JwksUseCase for EnvJwksUseCase {
    fn jwks(&self) -> BoxFuture<'_, Result<JwksOutput, JwksError>> {
        async { public_jwks_from_env().map_err(JwksError::new) }.boxed()
    }
}

fn jwt_expiration_seconds_from_env_value(value: Option<&str>) -> i64 {
    value
        .and_then(|raw| raw.trim().parse::<i64>().ok())
        .filter(|seconds| *seconds > 0)
        .unwrap_or(DEFAULT_JWT_EXPIRATION_SECONDS)
}

fn jwt_expiration_seconds() -> i64 {
    jwt_expiration_seconds_from_env_value(env::var("JWT_EXPIRATION_SECONDS").ok().as_deref())
}

fn jwt_key_id_from_env_value(value: Option<&str>) -> String {
    value
        .map(str::trim)
        .filter(|raw| !raw.is_empty())
        .unwrap_or(DEFAULT_JWT_KEY_ID)
        .to_string()
}

fn jwt_key_id() -> String {
    jwt_key_id_from_env_value(env::var("JWT_KEY_ID").ok().as_deref())
}

fn required_jwt_key_from_env(name: &str) -> Result<String, JwtError> {
    env::var(name).map_err(|_| ErrorKind::InvalidKeyFormat.into())
}

pub fn create_jwt(user_id: i32) -> Result<String, JwtError> {
    let private_key = required_jwt_key_from_env("PRIVATE_KEY")?;
    let expiration = chrono::Utc::now() + Duration::seconds(jwt_expiration_seconds());
    let claims = UserJWT::new(user_id, expiration);
    let encoding_key = EncodingKey::from_rsa_pem(private_key.as_bytes())?;
    let mut header = Header::new(Algorithm::RS256);
    header.kid = Some(jwt_key_id());
    encode(&header, &claims, &encoding_key)
}

pub fn decode_jwt(token: &str) -> Result<TokenData<UserJWT>, JwtError> {
    let public_key = required_jwt_key_from_env("PUBLIC_KEY")?;
    let decoding_key = DecodingKey::from_rsa_pem(public_key.as_bytes())?;
    let validation = Validation::new(Algorithm::RS256);
    decode::<UserJWT>(token, &decoding_key, &validation)
}

pub fn public_jwks_from_pem(
    public_key: &str,
    key_id: &str,
) -> Result<JwksOutput, openssl::error::ErrorStack> {
    let public_key = PKey::public_key_from_pem(public_key.as_bytes())?;
    let rsa = public_key.rsa()?;

    Ok(JwksOutput {
        keys: vec![JsonWebKeyOutput {
            kty: "RSA".to_string(),
            public_key_use: "sig".to_string(),
            kid: key_id.to_string(),
            alg: "RS256".to_string(),
            n: URL_SAFE_NO_PAD.encode(rsa.n().to_vec()),
            e: URL_SAFE_NO_PAD.encode(rsa.e().to_vec()),
        }],
    })
}

pub fn public_jwks_from_env() -> Result<JwksOutput, String> {
    let public_key = env::var("PUBLIC_KEY").map_err(|_| "PUBLIC_KEY must be set".to_string())?;
    public_jwks_from_pem(&public_key, &jwt_key_id()).map_err(|err| err.to_string())
}

#[cfg(test)]
mod tests;
