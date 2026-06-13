use std::collections::{BTreeMap, BTreeSet};

use crate::application::identity::current_session::{
    CourseSessionScope, CurrentSessionUser, DelegatedPermissionSession, OrganizationSessionScope,
    PlatformSessionScope,
};
use crate::models::delegated_permission::DelegatedPermission;
use crate::models::user::User;

#[derive(Debug, Default)]
pub(super) struct PlatformScopeBuilder {
    pub(super) roles: BTreeSet<String>,
    pub(super) direct_permissions: BTreeSet<String>,
    pub(super) delegated_permissions: BTreeSet<String>,
}

#[derive(Debug)]
pub(super) struct OrganizationScopeBuilder {
    id: i32,
    name: String,
    pub(super) roles: BTreeSet<String>,
    pub(super) direct_permissions: BTreeSet<String>,
    pub(super) delegated_permissions: BTreeSet<String>,
}

#[derive(Debug)]
pub(super) struct CourseScopeBuilder {
    id: i32,
    title: String,
    lifecycle_status: String,
    pub(super) roles: BTreeSet<String>,
    pub(super) direct_permissions: BTreeSet<String>,
    pub(super) delegated_permissions: BTreeSet<String>,
}

pub(super) fn organization_builder(
    organizations: &mut BTreeMap<i32, OrganizationScopeBuilder>,
    id: i32,
    name: String,
) -> &mut OrganizationScopeBuilder {
    organizations
        .entry(id)
        .or_insert_with(|| OrganizationScopeBuilder {
            id,
            name,
            roles: BTreeSet::new(),
            direct_permissions: BTreeSet::new(),
            delegated_permissions: BTreeSet::new(),
        })
}

pub(super) fn course_builder(
    courses: &mut BTreeMap<i32, CourseScopeBuilder>,
    id: i32,
    title: String,
    lifecycle_status: String,
) -> &mut CourseScopeBuilder {
    courses.entry(id).or_insert_with(|| CourseScopeBuilder {
        id,
        title,
        lifecycle_status,
        roles: BTreeSet::new(),
        direct_permissions: BTreeSet::new(),
        delegated_permissions: BTreeSet::new(),
    })
}

fn sorted_vec(values: BTreeSet<String>) -> Vec<String> {
    values.into_iter().collect()
}

fn effective_permissions(
    direct_permissions: &BTreeSet<String>,
    delegated_permissions: &BTreeSet<String>,
) -> Vec<String> {
    direct_permissions
        .union(delegated_permissions)
        .cloned()
        .collect()
}

impl From<User> for CurrentSessionUser {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            name: user.name,
            email: user.email,
            email_verified: user.email_verified,
            kyc_verified: user.kyc_verified,
        }
    }
}

impl From<PlatformScopeBuilder> for PlatformSessionScope {
    fn from(builder: PlatformScopeBuilder) -> Self {
        Self {
            roles: sorted_vec(builder.roles),
            effective_permissions: effective_permissions(
                &builder.direct_permissions,
                &builder.delegated_permissions,
            ),
            direct_permissions: sorted_vec(builder.direct_permissions),
            delegated_permissions: sorted_vec(builder.delegated_permissions),
        }
    }
}

impl From<OrganizationScopeBuilder> for OrganizationSessionScope {
    fn from(builder: OrganizationScopeBuilder) -> Self {
        Self {
            id: builder.id,
            name: builder.name,
            roles: sorted_vec(builder.roles),
            effective_permissions: effective_permissions(
                &builder.direct_permissions,
                &builder.delegated_permissions,
            ),
            direct_permissions: sorted_vec(builder.direct_permissions),
            delegated_permissions: sorted_vec(builder.delegated_permissions),
        }
    }
}

impl From<CourseScopeBuilder> for CourseSessionScope {
    fn from(builder: CourseScopeBuilder) -> Self {
        Self {
            id: builder.id,
            title: builder.title,
            lifecycle_status: builder.lifecycle_status,
            roles: sorted_vec(builder.roles),
            effective_permissions: effective_permissions(
                &builder.direct_permissions,
                &builder.delegated_permissions,
            ),
            direct_permissions: sorted_vec(builder.direct_permissions),
            delegated_permissions: sorted_vec(builder.delegated_permissions),
        }
    }
}

pub(super) fn delegated_permission_session(
    delegation: DelegatedPermission,
    organization_labels: &BTreeMap<i32, String>,
    course_labels: &BTreeMap<i32, (String, String)>,
) -> DelegatedPermissionSession {
    let (course_title, course_lifecycle_status) = delegation
        .course_id
        .and_then(|course_id| course_labels.get(&course_id).cloned())
        .map(|(title, lifecycle_status)| (Some(title), Some(lifecycle_status)))
        .unwrap_or((None, None));

    DelegatedPermissionSession {
        id: delegation.id,
        grantor_user_id: delegation.grantor_user_id,
        permission: delegation.permission,
        scope_type: delegation.scope_type,
        organization_id: delegation.organization_id,
        organization_name: delegation
            .organization_id
            .and_then(|organization_id| organization_labels.get(&organization_id).cloned()),
        course_id: delegation.course_id,
        course_title,
        course_lifecycle_status,
        expires_at: delegation.expires_at,
    }
}

#[cfg(test)]
mod tests;
