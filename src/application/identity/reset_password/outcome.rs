#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ResetPasswordOutcome {
    Reset,
    Expired,
    #[default]
    Invalid,
}
