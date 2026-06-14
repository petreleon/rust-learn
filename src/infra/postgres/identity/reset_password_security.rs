use bcrypt::{non_truncating_hash, DEFAULT_COST};

use crate::application::identity::reset_password::{ResetPasswordError, ResetPasswordHasher};

pub struct BcryptResetPasswordHasher;

impl ResetPasswordHasher for BcryptResetPasswordHasher {
    fn hash_password(&self, password: &str) -> Result<String, ResetPasswordError> {
        non_truncating_hash(password, DEFAULT_COST)
            .map_err(|error| ResetPasswordError::PasswordHash(error.to_string()))
    }
}
