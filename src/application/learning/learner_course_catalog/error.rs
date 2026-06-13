#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LearnerCourseCatalogError {
    PermissionDenied(String),
    NotFound,
    Connection(String),
    Database(String),
}
