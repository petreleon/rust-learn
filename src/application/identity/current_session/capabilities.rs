mod definitions;

use crate::application::identity::current_session::output::{
    CourseSessionScope, CurrentSessionAccess, OrganizationSessionScope, PlatformSessionScope,
    SessionCapability,
};
use definitions::CapabilityDefinition;

pub fn platform_capabilities(effective_permissions: &[String]) -> Vec<SessionCapability> {
    capabilities(definitions::PLATFORM, effective_permissions)
}

pub fn organization_capabilities(effective_permissions: &[String]) -> Vec<SessionCapability> {
    capabilities(definitions::ORGANIZATION, effective_permissions)
}

pub fn course_capabilities(effective_permissions: &[String]) -> Vec<SessionCapability> {
    capabilities(definitions::COURSE, effective_permissions)
}

pub fn access_summary(
    platform: &PlatformSessionScope,
    organizations: &[OrganizationSessionScope],
    courses: &[CourseSessionScope],
) -> CurrentSessionAccess {
    let teacher_application = has_permission(
        &platform.effective_permissions,
        "SUBMIT_TEACHER_APPLICATION",
    );
    let platform_admin = platform
        .capabilities
        .iter()
        .any(|capability| capability.enabled);
    let teacher_course = courses.iter().any(|course| {
        course
            .capabilities
            .iter()
            .any(|capability| capability.key == "teaching" && capability.enabled)
    });

    CurrentSessionAccess {
        learner: true,
        teacher: teacher_application || teacher_course,
        teacher_application,
        organization: organizations.iter().any(has_organization_access),
        platform_admin,
    }
}

fn capabilities(
    definitions: &[CapabilityDefinition],
    effective_permissions: &[String],
) -> Vec<SessionCapability> {
    definitions
        .iter()
        .map(|definition| SessionCapability {
            enabled: definition
                .permissions
                .iter()
                .any(|permission| has_permission(effective_permissions, permission)),
            key: definition.key.to_string(),
            label: definition.label.to_string(),
            permissions: definition
                .permissions
                .iter()
                .map(|permission| permission.to_string())
                .collect(),
        })
        .collect()
}

fn has_organization_access(organization: &OrganizationSessionScope) -> bool {
    !organization.roles.is_empty()
        || !organization.direct_permissions.is_empty()
        || !organization.delegated_permissions.is_empty()
        || organization
            .capabilities
            .iter()
            .any(|capability| capability.enabled)
}

fn has_permission(effective_permissions: &[String], permission: &str) -> bool {
    effective_permissions
        .iter()
        .any(|current| current == permission)
}
