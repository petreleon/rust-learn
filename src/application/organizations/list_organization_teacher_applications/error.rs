#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrganizationTeacherApplicationListError {
    PermissionDenied(String),
    InvalidInput(String),
    NotFound,
    Connection(String),
    Database(String),
}
