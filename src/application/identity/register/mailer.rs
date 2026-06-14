pub trait RegistrationEmailSender {
    fn send_verification_email(&self, email: &str, name: &str, token: &str);
}
