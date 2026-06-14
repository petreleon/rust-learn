use super::RegisterError;

pub trait RegistrationPasswordHasher {
    fn hash_password(&self, password: &str) -> Result<String, RegisterError>;
}
