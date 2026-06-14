#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResendVerificationError {
    Connection(String),
    Lookup(String),
    TokenGeneration(String),
    Store(String),
}
