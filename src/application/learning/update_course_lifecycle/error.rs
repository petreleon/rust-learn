#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CourseLifecycleError {
    PermissionDenied(String),
    InvalidStatus(String),
    InvalidTransition(String),
    StaleUpdate(String),
    NotFound,
    Connection(String),
    Database(String),
}
