#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrganizationMemberInviteOutput {
    pub user_id: i32,
    pub name: String,
    pub email: String,
    pub role: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrganizationMemberInviteTarget {
    pub user_id: i32,
    pub name: String,
    pub email: String,
}
