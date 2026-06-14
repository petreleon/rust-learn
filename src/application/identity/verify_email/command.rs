#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerifyEmailCommand {
    pub token: String,
}
