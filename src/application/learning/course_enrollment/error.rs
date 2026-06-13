#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CourseEnrollmentError {
    PermissionDenied(String),
    InvalidStatus(String),
    NotFound,
    Connection(String),
    Database(String),
}
