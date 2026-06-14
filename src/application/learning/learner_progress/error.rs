#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LearnerProgressError {
    PermissionDenied(String),
    NotFound,
    Connection(String),
    Database(String),
}
