#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlatformRoleAssignmentAuditError {
    Connection(String),
    Database(String),
    PermissionDenied(String),
    UserNotFound,
}
