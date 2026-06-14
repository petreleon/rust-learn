#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CourseLifecycleError {
    PermissionDenied(String),
    InvalidStatus(String),
    NotFound,
    Connection(String),
    Database(String),
}
