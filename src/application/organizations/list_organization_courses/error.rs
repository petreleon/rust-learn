#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrganizationCourseListError {
    PermissionDenied(String),
    NotFound,
    Connection(String),
    Database(String),
}
