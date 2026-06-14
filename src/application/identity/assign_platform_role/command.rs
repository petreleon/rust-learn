#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssignPlatformRoleCommand {
    pub requester_user_id: i32,
    pub target_user_id: i32,
    pub role_name: String,
}
