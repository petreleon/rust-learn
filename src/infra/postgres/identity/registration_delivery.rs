use crate::application::identity::register::{
    RegisterError, RegistrationEmailSender, RegistrationTokenGenerator,
};
use crate::utils::email::{generate_verification_token, print_mock_verification_email};

pub struct GeneratedRegistrationToken;

impl RegistrationTokenGenerator for GeneratedRegistrationToken {
    fn generate_token(&self) -> Result<String, RegisterError> {
        generate_verification_token()
            .map_err(|error| RegisterError::TokenGeneration(error.to_string()))
    }
}

pub struct MockRegistrationEmailSender;

impl RegistrationEmailSender for MockRegistrationEmailSender {
    fn send_verification_email(&self, email: &str, name: &str, token: &str) {
        print_mock_verification_email(email, name, token);
    }
}
