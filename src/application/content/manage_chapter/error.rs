#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChapterError {
    NotFound,
    Database(String),
}
