#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AssessmentAuthoringError {
    Connection(String),
    Database(String),
    NotFound,
    PermissionDenied(String),
    Validation(String),
}
