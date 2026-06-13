#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CourseUpdateError {
    PermissionDenied(String),
    NotFound,
    Connection(String),
    Database(String),
}
