use diesel_async::AsyncPgConnection;

use crate::application::organizations::list_organization_courses::{
    OrganizationCourseListError, OrganizationCoursePermissionSummaryOutput,
};
use crate::config::constants::permissions::Permissions;
use crate::repositories::organization_repository::user_permission_organization_request;
use crate::repositories::platform_repository::user_permission_platform_request;

pub async fn can_view_organization_courses(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    organization_id: i32,
) -> Result<bool, OrganizationCourseListError> {
    user_has_platform_or_organization_permission(
        conn,
        actor_user_id,
        organization_id,
        Permissions::VIEW_ORGANIZATION,
    )
    .await
}

pub async fn build_organization_course_permissions(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    organization_id: i32,
) -> Result<OrganizationCoursePermissionSummaryOutput, OrganizationCourseListError> {
    Ok(OrganizationCoursePermissionSummaryOutput {
        can_view_courses: can_view_organization_courses(conn, actor_user_id, organization_id)
            .await?,
        can_create_courses: user_has_platform_or_organization_permission(
            conn,
            actor_user_id,
            organization_id,
            Permissions::CREATE_COURSE,
        )
        .await?,
        can_manage_course_settings: user_has_platform_or_organization_permission(
            conn,
            actor_user_id,
            organization_id,
            Permissions::MANAGE_ORG_SETTINGS,
        )
        .await?
            || user_has_platform_or_organization_permission(
                conn,
                actor_user_id,
                organization_id,
                Permissions::MANAGE_COURSE_SETTINGS,
            )
            .await?,
        can_manage_enrollments: user_has_platform_or_organization_permission(
            conn,
            actor_user_id,
            organization_id,
            Permissions::APPROVE_COURSE_JOIN_REQUESTS,
        )
        .await?
            || user_has_platform_or_organization_permission(
                conn,
                actor_user_id,
                organization_id,
                Permissions::MANAGE_COURSE_ENROLLMENTS,
            )
            .await?,
        can_submit_reward_events: user_has_platform_or_organization_permission(
            conn,
            actor_user_id,
            organization_id,
            Permissions::SUBMIT_ORG_COURSE_REWARD_EVENT,
        )
        .await?,
        can_view_reward_reports: user_has_platform_or_organization_permission(
            conn,
            actor_user_id,
            organization_id,
            Permissions::VIEW_ORG_REWARD_REPORTS,
        )
        .await?,
        can_manage_reward_budget: user_has_platform_or_organization_permission(
            conn,
            actor_user_id,
            organization_id,
            Permissions::MANAGE_ORG_REWARD_BUDGET,
        )
        .await?,
    })
}

async fn user_has_platform_or_organization_permission(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    organization_id: i32,
    permission: Permissions,
) -> Result<bool, OrganizationCourseListError> {
    let permission_name = permission.to_string();
    if user_permission_platform_request(conn, actor_user_id, &permission_name)
        .await
        .map_err(map_organization_error)?
    {
        return Ok(true);
    }

    user_permission_organization_request(conn, actor_user_id, organization_id, &permission_name)
        .await
        .map_err(map_organization_error)
}

fn map_organization_error(error: diesel::result::Error) -> OrganizationCourseListError {
    match error {
        diesel::result::Error::NotFound => OrganizationCourseListError::NotFound,
        other => OrganizationCourseListError::Database(other.to_string()),
    }
}
