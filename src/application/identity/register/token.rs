use super::RegisterError;

pub trait RegistrationTokenGenerator {
    fn generate_token(&self) -> Result<String, RegisterError>;
}
