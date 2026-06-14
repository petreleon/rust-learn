pub trait PasswordVerifier {
    fn verify_password(&self, password: &str, password_hash: &str) -> bool;
}
