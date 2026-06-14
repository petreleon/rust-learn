#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequestPasswordResetOutcome {
    Sent,
    UnknownEmail,
}
