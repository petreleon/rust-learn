#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegisterError {
    InvalidPassword(String),
    Connection(String),
    PasswordHash(String),
    TokenGeneration(String),
    EmailAlreadyRegistered,
    DefaultStudentRoleMissing,
    Store(String),
}
