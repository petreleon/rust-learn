use bcrypt::verify;

use crate::application::identity::login::{LoginError, LoginTokenIssuer, PasswordVerifier};

#[derive(Clone, Copy)]
pub struct BcryptPasswordVerifier;

impl PasswordVerifier for BcryptPasswordVerifier {
    fn verify_password(&self, password: &str, password_hash: &str) -> bool {
        verify(password, password_hash).unwrap_or(false)
    }
}

#[derive(Clone, Copy)]
pub struct JwtLoginTokenIssuer;

impl LoginTokenIssuer for JwtLoginTokenIssuer {
    fn issue_token(&self, user_id: i32) -> Result<String, LoginError> {
        crate::utils::jwt_utils::create_jwt(user_id).map_err(|error| LoginError::Token {
            user_id,
            message: error.to_string(),
        })
    }
}
