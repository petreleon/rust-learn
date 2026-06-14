#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CourseReadError {
    NotFound,
    Connection(String),
    Database(String),
}
