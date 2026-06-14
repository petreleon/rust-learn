use super::RequestPasswordResetError;

pub trait PasswordResetTokenGenerator {
    fn generate_token(&self) -> Result<String, RequestPasswordResetError>;
}
