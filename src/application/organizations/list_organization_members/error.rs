#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrganizationMemberListError {
    PermissionDenied,
    NotFound,
    Connection(String),
    Database(String),
}
