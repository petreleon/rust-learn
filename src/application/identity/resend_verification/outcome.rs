#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResendVerificationOutcome {
    Sent,
    AlreadyVerified,
    UnknownEmail,
}
