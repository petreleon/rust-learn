use crate::application::identity::resend_verification::{
    ResendVerificationError, VerificationEmailSender, VerificationTokenGenerator,
};
use crate::infra::email::identity::print_mock_verification_email;
use crate::utils::email::generate_verification_token;

pub struct GeneratedVerificationToken;

impl VerificationTokenGenerator for GeneratedVerificationToken {
    fn generate_token(&self) -> Result<String, ResendVerificationError> {
        generate_verification_token()
            .map_err(|error| ResendVerificationError::TokenGeneration(error.to_string()))
    }
}

pub struct MockVerificationEmailSender;

impl VerificationEmailSender for MockVerificationEmailSender {
    fn send_verification_email(&self, email: &str, name: &str, token: &str) {
        print_mock_verification_email(email, name, token);
    }
}
