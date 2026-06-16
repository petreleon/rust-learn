#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CourseCompletionTermsError {
    PermissionDenied(String),
    InvalidInput(String),
    InvalidStatus(String),
    CourseNotFound,
    TermsNotFound,
    Connection(String),
    Database(String),
}
