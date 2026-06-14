use crate::config::constants::permissions::Permissions;
use diesel_async::AsyncPgConnection;

use super::errors::{OrganizationCourseListError, TeacherCourseDashboardError};
use super::learner_permissions::user_has_permission_for_course_context;
use super::teacher_delegated_scope::user_has_platform_or_organization_permission;
use super::teacher_enrollment_types::{
    OrganizationCoursePermissionSummary, TeacherCoursePermissionSummary,
};

pub(super) async fn build_organization_course_permissions(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    organization_id: i32,
) -> Result<OrganizationCoursePermissionSummary, OrganizationCourseListError> {
    Ok(OrganizationCoursePermissionSummary {
        can_view_courses: user_has_platform_or_organization_permission(
            conn,
            actor_user_id,
            organization_id,
            Permissions::VIEW_ORGANIZATION,
        )
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

pub(super) async fn build_teacher_course_permissions(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    course_id: i32,
) -> Result<TeacherCoursePermissionSummary, TeacherCourseDashboardError> {
    let can_create_content =
        teacher_has_course_permission(conn, actor_user_id, course_id, Permissions::CREATE_CONTENT)
            .await?;
    let can_modify_content =
        teacher_has_course_permission(conn, actor_user_id, course_id, Permissions::MODIFY_CONTENT)
            .await?;
    let can_approve_content = teacher_has_course_permission(
        conn,
        actor_user_id,
        course_id,
        Permissions::APPROVE_COURSE_CONTENT,
    )
    .await?;

    Ok(TeacherCoursePermissionSummary {
        can_manage_settings: teacher_has_course_permission(
            conn,
            actor_user_id,
            course_id,
            Permissions::MANAGE_COURSE_SETTINGS,
        )
        .await?,
        can_manage_content: can_create_content || can_modify_content || can_approve_content,
        can_manage_enrollments: teacher_has_course_permission(
            conn,
            actor_user_id,
            course_id,
            Permissions::MANAGE_COURSE_ENROLLMENTS,
        )
        .await?
            || teacher_has_course_permission(
                conn,
                actor_user_id,
                course_id,
                Permissions::APPROVE_COURSE_JOIN_REQUESTS,
            )
            .await?,
        can_view_reward_candidates: teacher_has_course_permission(
            conn,
            actor_user_id,
            course_id,
            Permissions::VIEW_COURSE_REWARD_STATUS,
        )
        .await?,
        can_approve_reward_candidates: teacher_has_course_permission(
            conn,
            actor_user_id,
            course_id,
            Permissions::APPROVE_STUDENT_REWARD_CANDIDATE,
        )
        .await?,
        can_manage_reward_rules: teacher_has_course_permission(
            conn,
            actor_user_id,
            course_id,
            Permissions::MANAGE_COURSE_REWARD_RULES,
        )
        .await?,
    })
}

async fn teacher_has_course_permission(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    course_id: i32,
    permission: Permissions,
) -> Result<bool, TeacherCourseDashboardError> {
    user_has_permission_for_course_context(conn, actor_user_id, course_id, &permission)
        .await
        .map_err(TeacherCourseDashboardError::from)
}
