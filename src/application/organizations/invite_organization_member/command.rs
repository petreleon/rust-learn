#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrganizationMemberInviteCommand {
    pub actor_user_id: i32,
    pub organization_id: i32,
    pub email: String,
    pub role_name: Option<String>,
}

impl OrganizationMemberInviteCommand {
    pub fn lookup_email(&self) -> String {
        self.email.trim().to_string()
    }

    pub fn requested_role_name(&self) -> String {
        self.role_name
            .clone()
            .unwrap_or_else(|| "STUDENT".to_string())
    }
}
