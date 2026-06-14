#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContentItemError {
    ChapterNotFound,
    ContentNotFound,
    Connection(String),
    Database(String),
}
