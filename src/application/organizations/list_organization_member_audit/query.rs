#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OrganizationMemberAuditQuery {
    pub actor_user_id: i32,
    pub organization_id: i32,
    pub target_user_id: i32,
}
