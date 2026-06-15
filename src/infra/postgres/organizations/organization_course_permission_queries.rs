use diesel_async::AsyncPgConnection;

use crate::application::access_control::check_permission::{
    AccessAction, AccessActor, AccessScope,
};
use crate::application::organizations::list_organization_courses::{
    OrganizationCourseListError, OrganizationCoursePermissionSummaryOutput,
};
use crate::domain::access_control::permissions::Permissions;
use crate::infra::postgres::access_control::permission_checks;

pub async fn can_view_organization_courses(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    organization_id: i32,
) -> Result<bool, OrganizationCourseListError> {
    can_organization_operator(
        conn,
        actor_user_id,
        organization_id,
        Permissions::VIEW_ORGANIZATION,
    )
    .await
}

async fn can_organization_operator(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    organization_id: i32,
    permission: Permissions,
) -> Result<bool, OrganizationCourseListError> {
    permission_checks::can_any(
        conn,
        AccessActor::user(actor_user_id),
        AccessAction::permission(permission.to_string()),
        &[
            AccessScope::platform(),
            AccessScope::organization(organization_id),
        ],
    )
    .await
    .map_err(map_organization_error)
}

pub async fn build_organization_course_permissions(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    organization_id: i32,
) -> Result<OrganizationCoursePermissionSummaryOutput, OrganizationCourseListError> {
    Ok(OrganizationCoursePermissionSummaryOutput {
        can_view_courses: can_view_organization_courses(conn, actor_user_id, organization_id)
            .await?,
        can_create_courses: can_organization_operator(
            conn,
            actor_user_id,
            organization_id,
            Permissions::CREATE_COURSE,
        )
        .await?,
        can_manage_course_settings: can_organization_operator(
            conn,
            actor_user_id,
            organization_id,
            Permissions::MANAGE_ORG_SETTINGS,
        )
        .await?
            || can_organization_operator(
                conn,
                actor_user_id,
                organization_id,
                Permissions::MANAGE_COURSE_SETTINGS,
            )
            .await?,
        can_manage_enrollments: can_organization_operator(
            conn,
            actor_user_id,
            organization_id,
            Permissions::APPROVE_COURSE_JOIN_REQUESTS,
        )
        .await?
            || can_organization_operator(
                conn,
                actor_user_id,
                organization_id,
                Permissions::MANAGE_COURSE_ENROLLMENTS,
            )
            .await?,
        can_submit_reward_events: can_organization_operator(
            conn,
            actor_user_id,
            organization_id,
            Permissions::SUBMIT_ORG_COURSE_REWARD_EVENT,
        )
        .await?,
        can_view_reward_reports: can_organization_operator(
            conn,
            actor_user_id,
            organization_id,
            Permissions::VIEW_ORG_REWARD_REPORTS,
        )
        .await?,
        can_manage_reward_budget: can_organization_operator(
            conn,
            actor_user_id,
            organization_id,
            Permissions::MANAGE_ORG_REWARD_BUDGET,
        )
        .await?,
    })
}

fn map_organization_error(error: diesel::result::Error) -> OrganizationCourseListError {
    match error {
        diesel::result::Error::NotFound => OrganizationCourseListError::NotFound,
        other => OrganizationCourseListError::Database(other.to_string()),
    }
}
