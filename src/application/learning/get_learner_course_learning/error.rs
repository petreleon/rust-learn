#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LearnerCourseLearningError {
    PermissionDenied(String),
    NotFound,
    Connection(String),
    Database(String),
}
