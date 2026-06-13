#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CourseCreationError {
    PermissionDenied(String),
    Connection(String),
    Database(String),
}
