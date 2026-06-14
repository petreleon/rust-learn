#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RequestPasswordResetError {
    Connection(String),
    Lookup(String),
    TokenGeneration(String),
    Store(String),
}
