#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResetPasswordError {
    MissingToken,
    InvalidPassword(String),
    Connection(String),
    PasswordHash(String),
    Store(String),
}
