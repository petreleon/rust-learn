#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrganizationMemberInviteError {
    PermissionDenied,
    UserNotFound,
    HierarchyDenied,
    RoleOrUserNotFound,
    Connection(String),
    Database(String),
}
