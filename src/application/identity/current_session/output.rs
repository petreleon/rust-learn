use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CurrentSessionOutput {
    pub access: CurrentSessionAccess,
    pub user: CurrentSessionUser,
    pub platform: PlatformSessionScope,
    pub organizations: Vec<OrganizationSessionScope>,
    pub courses: Vec<CourseSessionScope>,
    pub delegated_permissions: Vec<DelegatedPermissionSession>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CurrentSessionUser {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub email_verified: bool,
    pub kyc_verified: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CurrentSessionAccess {
    pub learner: bool,
    pub teacher: bool,
    pub teacher_application: bool,
    pub organization: bool,
    pub platform_admin: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionCapability {
    pub key: String,
    pub label: String,
    pub permissions: Vec<String>,
    pub enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlatformSessionScope {
    pub roles: Vec<String>,
    pub direct_permissions: Vec<String>,
    pub delegated_permissions: Vec<String>,
    pub effective_permissions: Vec<String>,
    pub capabilities: Vec<SessionCapability>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrganizationSessionScope {
    pub id: i32,
    pub name: String,
    pub roles: Vec<String>,
    pub direct_permissions: Vec<String>,
    pub delegated_permissions: Vec<String>,
    pub effective_permissions: Vec<String>,
    pub capabilities: Vec<SessionCapability>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CourseSessionScope {
    pub id: i32,
    pub title: String,
    pub lifecycle_status: String,
    pub roles: Vec<String>,
    pub direct_permissions: Vec<String>,
    pub delegated_permissions: Vec<String>,
    pub effective_permissions: Vec<String>,
    pub capabilities: Vec<SessionCapability>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DelegatedPermissionSession {
    pub id: i64,
    pub grantor_user_id: i32,
    pub permission: String,
    pub scope_type: String,
    pub organization_id: Option<i32>,
    pub organization_name: Option<String>,
    pub course_id: Option<i32>,
    pub course_title: Option<String>,
    pub course_lifecycle_status: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
}
