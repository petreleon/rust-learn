use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::application::identity::current_session::{
    CourseSessionScope, CurrentSessionAccess, CurrentSessionUser, DelegatedPermissionSession,
    OrganizationSessionScope, PlatformSessionScope, SessionCapability,
};

#[derive(Debug, Clone, Serialize)]
pub(super) struct CurrentSessionUserResponse {
    id: i32,
    name: String,
    email: String,
    email_verified: bool,
    kyc_verified: bool,
}

#[derive(Debug, Clone, Serialize)]
pub(super) struct CurrentSessionAccessResponse {
    learner: bool,
    teacher: bool,
    teacher_application: bool,
    organization: bool,
    platform_admin: bool,
}

#[derive(Debug, Clone, Serialize)]
pub(super) struct SessionCapabilityResponse {
    key: String,
    label: String,
    permissions: Vec<String>,
    enabled: bool,
}

#[derive(Debug, Clone, Serialize)]
pub(super) struct PlatformSessionScopeResponse {
    roles: Vec<String>,
    direct_permissions: Vec<String>,
    delegated_permissions: Vec<String>,
    effective_permissions: Vec<String>,
    capabilities: Vec<SessionCapabilityResponse>,
}

#[derive(Debug, Clone, Serialize)]
pub(super) struct OrganizationSessionScopeResponse {
    id: i32,
    name: String,
    roles: Vec<String>,
    direct_permissions: Vec<String>,
    delegated_permissions: Vec<String>,
    effective_permissions: Vec<String>,
    capabilities: Vec<SessionCapabilityResponse>,
}

#[derive(Debug, Clone, Serialize)]
pub(super) struct CourseSessionScopeResponse {
    id: i32,
    title: String,
    lifecycle_status: String,
    roles: Vec<String>,
    direct_permissions: Vec<String>,
    delegated_permissions: Vec<String>,
    effective_permissions: Vec<String>,
    capabilities: Vec<SessionCapabilityResponse>,
}

#[derive(Debug, Clone, Serialize)]
pub(super) struct DelegatedPermissionSessionResponse {
    id: i64,
    grantor_user_id: i32,
    permission: String,
    scope_type: String,
    organization_id: Option<i32>,
    organization_name: Option<String>,
    course_id: Option<i32>,
    course_title: Option<String>,
    course_lifecycle_status: Option<String>,
    expires_at: Option<DateTime<Utc>>,
}

impl From<CurrentSessionAccess> for CurrentSessionAccessResponse {
    fn from(access: CurrentSessionAccess) -> Self {
        Self {
            learner: access.learner,
            teacher: access.teacher,
            teacher_application: access.teacher_application,
            organization: access.organization,
            platform_admin: access.platform_admin,
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
            capabilities: scope.capabilities.into_iter().map(Into::into).collect(),
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
            capabilities: scope.capabilities.into_iter().map(Into::into).collect(),
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
            capabilities: scope.capabilities.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<SessionCapability> for SessionCapabilityResponse {
    fn from(capability: SessionCapability) -> Self {
        Self {
            key: capability.key,
            label: capability.label,
            permissions: capability.permissions,
            enabled: capability.enabled,
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
