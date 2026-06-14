use super::ResendVerificationError;

pub trait VerificationTokenGenerator {
    fn generate_token(&self) -> Result<String, ResendVerificationError>;
}
