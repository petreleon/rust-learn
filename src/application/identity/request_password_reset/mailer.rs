pub trait PasswordResetEmailSender {
    fn send_password_reset_email(&self, email: &str, name: &str, token: &str);
}
