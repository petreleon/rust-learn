use super::{
    jwt_expiration_seconds_from_env_value, jwt_key_id_from_env_value, public_jwks_from_pem,
    required_jwt_key_from_env, DEFAULT_JWT_EXPIRATION_SECONDS, DEFAULT_JWT_KEY_ID,
};
use jsonwebtoken::errors::ErrorKind;
use openssl::pkey::PKey;
use openssl::rsa::Rsa;

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

#[test]
fn uses_default_jwt_key_id_when_unset_or_blank() {
    assert_eq!(jwt_key_id_from_env_value(None), DEFAULT_JWT_KEY_ID);
    assert_eq!(jwt_key_id_from_env_value(Some("   ")), DEFAULT_JWT_KEY_ID);
}

#[test]
fn parses_jwt_key_id() {
    assert_eq!(jwt_key_id_from_env_value(Some(" primary ")), "primary");
}

#[test]
fn missing_jwt_key_env_returns_key_error() {
    let err = required_jwt_key_from_env("RUST_LEARN_TEST_MISSING_JWT_KEY")
        .expect_err("missing key should be returned as a JWT error");

    assert_eq!(err.kind(), &ErrorKind::InvalidKeyFormat);
}

#[test]
fn builds_jwks_from_rsa_public_key() {
    let rsa = Rsa::generate(2048).expect("failed to generate rsa key");
    let key_pair = PKey::from_rsa(rsa).expect("failed to create key pair");
    let public_key = String::from_utf8(
        key_pair
            .public_key_to_pem()
            .expect("failed to export public key"),
    )
    .expect("public key should be utf-8");

    let jwks = public_jwks_from_pem(&public_key, "test-key").expect("failed to build jwks");
    let key = jwks.keys.first().expect("jwks should include a key");

    assert_eq!(key.kty, "RSA");
    assert_eq!(key.public_key_use, "sig");
    assert_eq!(key.kid, "test-key");
    assert_eq!(key.alg, "RS256");
    assert!(!key.n.is_empty());
    assert_eq!(key.e, "AQAB");
}
