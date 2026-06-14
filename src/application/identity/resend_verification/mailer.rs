pub trait VerificationEmailSender {
    fn send_verification_email(&self, email: &str, name: &str, token: &str);
}
