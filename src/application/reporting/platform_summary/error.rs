#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlatformSummaryError {
    Connection(String),
    Database(String),
}
