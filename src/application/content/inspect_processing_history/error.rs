#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContentProcessingHistoryError {
    ChapterNotFound,
    ContentNotFound,
    Connection(String),
    Database(String),
}
