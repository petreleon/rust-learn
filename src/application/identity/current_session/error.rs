#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CurrentSessionError {
    MissingUser,
    EmailUnverified,
    Connection(String),
    Database(String),
}
