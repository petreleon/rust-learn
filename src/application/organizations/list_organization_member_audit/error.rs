#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrganizationMemberAuditError {
    PermissionDenied(String),
    Connection(String),
    Database(String),
}
