#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VerifyEmailOutcome {
    Verified,
    AlreadyVerified,
    Expired,
    Invalid,
}
