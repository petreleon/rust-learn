#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResetPasswordCommand {
    pub token: String,
    pub password: String,
}
