#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrganizationMemberRemovalError {
    PermissionDenied,
    NotFound,
    Connection(String),
    Database(String),
}
