use super::ResetPasswordError;

pub trait ResetPasswordHasher {
    fn hash_password(&self, password: &str) -> Result<String, ResetPasswordError>;
}
