use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::application::identity::current_session::{
    CourseSessionScope, CurrentSessionOutput, CurrentSessionUser, DelegatedPermissionSession,
    OrganizationSessionScope, PlatformSessionScope,
};

#[derive(Debug, Clone, Serialize)]
pub struct CurrentSessionResponse {
    pub user: CurrentSessionUserResponse,
    pub platform: PlatformSessionScopeResponse,
    pub organizations: Vec<OrganizationSessionScopeResponse>,
    pub courses: Vec<CourseSessionScopeResponse>,
    pub delegated_permissions: Vec<DelegatedPermissionSessionResponse>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CurrentSessionUserResponse {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub email_verified: bool,
    pub kyc_verified: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct PlatformSessionScopeResponse {
    pub roles: Vec<String>,
    pub direct_permissions: Vec<String>,
    pub delegated_permissions: Vec<String>,
    pub effective_permissions: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct OrganizationSessionScopeResponse {
    pub id: i32,
    pub name: String,
    pub roles: Vec<String>,
    pub direct_permissions: Vec<String>,
    pub delegated_permissions: Vec<String>,
    pub effective_permissions: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CourseSessionScopeResponse {
    pub id: i32,
    pub title: String,
    pub lifecycle_status: String,
    pub roles: Vec<String>,
    pub direct_permissions: Vec<String>,
    pub delegated_permissions: Vec<String>,
    pub effective_permissions: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DelegatedPermissionSessionResponse {
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

impl From<CurrentSessionOutput> for CurrentSessionResponse {
    fn from(session: CurrentSessionOutput) -> Self {
        Self {
            user: session.user.into(),
            platform: session.platform.into(),
            organizations: session.organizations.into_iter().map(Into::into).collect(),
            courses: session.courses.into_iter().map(Into::into).collect(),
            delegated_permissions: session
                .delegated_permissions
                .into_iter()
                .map(Into::into)
                .collect(),
        }
    }
}

impl From<CurrentSessionUser> for CurrentSessionUserResponse {
    fn from(user: CurrentSessionUser) -> Self {
        Self {
            id: user.id,
            name: user.name,
            email: user.email,
            email_verified: user.email_verified,
            kyc_verified: user.kyc_verified,
        }
    }
}

impl From<PlatformSessionScope> for PlatformSessionScopeResponse {
    fn from(scope: PlatformSessionScope) -> Self {
        Self {
            roles: scope.roles,
            direct_permissions: scope.direct_permissions,
            delegated_permissions: scope.delegated_permissions,
            effective_permissions: scope.effective_permissions,
        }
    }
}

impl From<OrganizationSessionScope> for OrganizationSessionScopeResponse {
    fn from(scope: OrganizationSessionScope) -> Self {
        Self {
            id: scope.id,
            name: scope.name,
            roles: scope.roles,
            direct_permissions: scope.direct_permissions,
            delegated_permissions: scope.delegated_permissions,
            effective_permissions: scope.effective_permissions,
        }
    }
}

impl From<CourseSessionScope> for CourseSessionScopeResponse {
    fn from(scope: CourseSessionScope) -> Self {
        Self {
            id: scope.id,
            title: scope.title,
            lifecycle_status: scope.lifecycle_status,
            roles: scope.roles,
            direct_permissions: scope.direct_permissions,
            delegated_permissions: scope.delegated_permissions,
            effective_permissions: scope.effective_permissions,
        }
    }
}

impl From<DelegatedPermissionSession> for DelegatedPermissionSessionResponse {
    fn from(delegation: DelegatedPermissionSession) -> Self {
        Self {
            id: delegation.id,
            grantor_user_id: delegation.grantor_user_id,
            permission: delegation.permission,
            scope_type: delegation.scope_type,
            organization_id: delegation.organization_id,
            organization_name: delegation.organization_name,
            course_id: delegation.course_id,
            course_title: delegation.course_title,
            course_lifecycle_status: delegation.course_lifecycle_status,
            expires_at: delegation.expires_at,
        }
    }
}
