#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlatformRoleAssignmentAuditQuery {
    pub actor_user_id: i32,
    pub target_user_id: i32,
}
