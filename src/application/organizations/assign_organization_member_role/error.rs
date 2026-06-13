#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrganizationMemberRoleAssignmentError {
    PermissionDenied,
    HierarchyViolation,
    NotFound,
    Connection(String),
    Database(String),
}
