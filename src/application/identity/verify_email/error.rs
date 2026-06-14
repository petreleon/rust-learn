#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VerifyEmailError {
    Connection(String),
    Database(String),
}
