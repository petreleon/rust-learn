#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequestPasswordResetCommand {
    pub email: String,
}
