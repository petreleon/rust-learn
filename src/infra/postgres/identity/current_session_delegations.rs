use std::collections::{BTreeMap, BTreeSet};

use diesel::result::Error as DieselError;
use diesel_async::AsyncPgConnection;

use crate::application::identity::current_session::CurrentSessionError;
use crate::infra::postgres::identity::current_session_scope_builder::{
    course_builder, organization_builder, CourseScopeBuilder, OrganizationScopeBuilder,
    PlatformScopeBuilder,
};
use crate::models::delegated_permission::DelegatedPermission;
use crate::repositories::session_repository;

pub(super) async fn organization_labels(
    conn: &mut AsyncPgConnection,
    delegations: &[DelegatedPermission],
) -> Result<BTreeMap<i32, String>, CurrentSessionError> {
    let ids = delegations
        .iter()
        .filter_map(|delegation| delegation.organization_id)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();

    session_repository::organization_labels(conn, &ids)
        .await
        .map(|rows| rows.into_iter().collect())
        .map_err(map_current_session_error)
}

pub(super) async fn course_labels(
    conn: &mut AsyncPgConnection,
    delegations: &[DelegatedPermission],
) -> Result<BTreeMap<i32, (String, String)>, CurrentSessionError> {
    let ids = delegations
        .iter()
        .filter_map(|delegation| delegation.course_id)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();

    session_repository::course_labels(conn, &ids)
        .await
        .map(|rows| {
            rows.into_iter()
                .map(|(id, title, status)| (id, (title, status)))
                .collect()
        })
        .map_err(map_current_session_error)
}

pub(super) fn apply_delegations(
    delegations: &[DelegatedPermission],
    organization_labels: &BTreeMap<i32, String>,
    course_labels: &BTreeMap<i32, (String, String)>,
    platform: &mut PlatformScopeBuilder,
    organizations: &mut BTreeMap<i32, OrganizationScopeBuilder>,
    courses: &mut BTreeMap<i32, CourseScopeBuilder>,
) {
    for delegation in delegations {
        match delegation.scope_type.as_str() {
            "platform" => {
                platform
                    .delegated_permissions
                    .insert(delegation.permission.clone());
            }
            "organization" => {
                apply_organization_delegation(delegation, organization_labels, organizations)
            }
            "course" => apply_course_delegation(delegation, course_labels, courses),
            _ => {}
        }
    }
}

fn apply_organization_delegation(
    delegation: &DelegatedPermission,
    organization_labels: &BTreeMap<i32, String>,
    organizations: &mut BTreeMap<i32, OrganizationScopeBuilder>,
) {
    let Some(organization_id) = delegation.organization_id else {
        return;
    };
    let organization_name = organization_labels
        .get(&organization_id)
        .cloned()
        .unwrap_or_else(|| format!("Organization {organization_id}"));

    organization_builder(organizations, organization_id, organization_name)
        .delegated_permissions
        .insert(delegation.permission.clone());
}

fn apply_course_delegation(
    delegation: &DelegatedPermission,
    course_labels: &BTreeMap<i32, (String, String)>,
    courses: &mut BTreeMap<i32, CourseScopeBuilder>,
) {
    let Some(course_id) = delegation.course_id else {
        return;
    };
    let (course_title, lifecycle_status) = course_labels
        .get(&course_id)
        .cloned()
        .unwrap_or_else(|| (format!("Course {course_id}"), "unknown".to_string()));

    course_builder(courses, course_id, course_title, lifecycle_status)
        .delegated_permissions
        .insert(delegation.permission.clone());
}

fn map_current_session_error(error: DieselError) -> CurrentSessionError {
    match error {
        DieselError::NotFound => CurrentSessionError::MissingUser,
        other => CurrentSessionError::Database(other.to_string()),
    }
}
