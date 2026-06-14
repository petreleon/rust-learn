#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChapterError {
    NotFound,
    Connection(String),
    Database(String),
}
