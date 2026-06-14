#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrganizationMemberRoleAssignmentOutput {
    pub organization_id: i32,
    pub target_user_id: i32,
    pub role_name: String,
}
