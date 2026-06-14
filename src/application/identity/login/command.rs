#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoginCommand {
    pub email: String,
    pub password: String,
}
