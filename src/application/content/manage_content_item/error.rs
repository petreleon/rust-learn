#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContentItemError {
    ChapterNotFound,
    ContentNotFound,
    Database(String),
}
