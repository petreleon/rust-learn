#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrganizationMemberRoleAssignmentCommand {
    pub actor_user_id: i32,
    pub organization_id: i32,
    pub target_user_id: i32,
    pub role_name: String,
}
