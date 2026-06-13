#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrganizationMemberListOutput {
    pub organization: OrganizationMemberOrganizationOutput,
    pub members: Vec<OrganizationMemberListItemOutput>,
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
    pub search: Option<String>,
    pub role: Option<String>,
    pub permission: Option<String>,
    pub operator_permissions: OrganizationMemberOperatorPermissionsOutput,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrganizationMemberOrganizationOutput {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrganizationMemberListItemOutput {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub email_verified: bool,
    pub kyc_verified: bool,
    pub joined_at: String,
    pub roles: Vec<String>,
    pub direct_permissions: Vec<String>,
    pub delegated_permissions: Vec<String>,
    pub effective_permissions: Vec<String>,
    pub direct_permission_count: usize,
    pub delegated_permission_count: usize,
    pub effective_permission_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrganizationMemberOperatorPermissionsOutput {
    pub can_view_members: bool,
    pub can_invite_members: bool,
    pub can_manage_members: bool,
    pub can_assign_roles: bool,
    pub can_manage_settings: bool,
}
