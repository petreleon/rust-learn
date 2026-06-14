#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AssignPlatformRoleError {
    Connection(String),
    Database(String),
    HierarchyViolation,
    RoleNotFound,
}
