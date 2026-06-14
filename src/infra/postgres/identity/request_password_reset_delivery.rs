use crate::application::identity::request_password_reset::{
    PasswordResetEmailSender, PasswordResetTokenGenerator, RequestPasswordResetError,
};
use crate::infra::email::identity::print_mock_password_reset_email;
use crate::utils::email::generate_verification_token;

pub struct GeneratedPasswordResetToken;

impl PasswordResetTokenGenerator for GeneratedPasswordResetToken {
    fn generate_token(&self) -> Result<String, RequestPasswordResetError> {
        generate_verification_token()
            .map_err(|error| RequestPasswordResetError::TokenGeneration(error.to_string()))
    }
}

pub struct MockPasswordResetEmailSender;

impl PasswordResetEmailSender for MockPasswordResetEmailSender {
    fn send_password_reset_email(&self, email: &str, name: &str, token: &str) {
        print_mock_password_reset_email(email, name, token);
    }
}
