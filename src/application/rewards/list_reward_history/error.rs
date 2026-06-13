#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StudentRewardHistoryError {
    InvalidInput(String),
    Connection(String),
    Database(String),
}
