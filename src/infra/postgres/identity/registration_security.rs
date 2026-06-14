use bcrypt::{non_truncating_hash, DEFAULT_COST};

use crate::application::identity::register::{RegisterError, RegistrationPasswordHasher};

pub struct BcryptRegistrationPasswordHasher;

impl RegistrationPasswordHasher for BcryptRegistrationPasswordHasher {
    fn hash_password(&self, password: &str) -> Result<String, RegisterError> {
        non_truncating_hash(password, DEFAULT_COST)
            .map_err(|error| RegisterError::PasswordHash(error.to_string()))
    }
}
