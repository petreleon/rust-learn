#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StudentRewardHistoryError {
    InvalidInput(String),
    InvalidStatus(String),
    Connection(String),
    Database(String),
}
