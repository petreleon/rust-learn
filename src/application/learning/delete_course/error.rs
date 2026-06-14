#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CourseDeletionError {
    Connection(String),
    Database(String),
}
