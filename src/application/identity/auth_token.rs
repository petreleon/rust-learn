use std::sync::Arc;

use crate::domain::identity::UserJWT;

pub type AuthTokenVerifierService = Arc<dyn AuthTokenVerifier>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthTokenVerificationError {
    Expired,
    Invalid,
}

pub trait AuthTokenVerifier: Send + Sync {
    fn verify_token(&self, token: &str) -> Result<UserJWT, AuthTokenVerificationError>;
}
