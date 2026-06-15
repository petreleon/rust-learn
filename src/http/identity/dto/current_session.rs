mod scope_response;

use scope_response::{
    CourseSessionScopeResponse, CurrentSessionAccessResponse, CurrentSessionUserResponse,
    DelegatedPermissionSessionResponse, OrganizationSessionScopeResponse,
    PlatformSessionScopeResponse,
};
use serde::Serialize;

use crate::application::identity::current_session::CurrentSessionOutput;

#[derive(Debug, Clone, Serialize)]
pub struct CurrentSessionResponse {
    access: CurrentSessionAccessResponse,
    user: CurrentSessionUserResponse,
    platform: PlatformSessionScopeResponse,
    organizations: Vec<OrganizationSessionScopeResponse>,
    courses: Vec<CourseSessionScopeResponse>,
    delegated_permissions: Vec<DelegatedPermissionSessionResponse>,
}

impl From<CurrentSessionOutput> for CurrentSessionResponse {
    fn from(session: CurrentSessionOutput) -> Self {
        Self {
            access: session.access.into(),
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
