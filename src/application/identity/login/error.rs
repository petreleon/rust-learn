#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoginError {
    Connection(String),
    EmailUnverified { user_id: i32 },
    InvalidCredentials,
    MissingPasswordAuthentication,
    Token { user_id: i32, message: String },
}
