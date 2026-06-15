use diesel_async::AsyncPgConnection;

use crate::application::access_control::check_permission::{
    AccessAction, AccessActor, AccessScope,
};
use crate::application::learning::teacher_course_dashboard::{
    TeacherCourseDashboardError, TeacherCoursePermissionSummaryOutput,
};
use crate::domain::access_control::permissions::Permissions;
use crate::infra::postgres::access_control::permission_checks;

pub fn teacher_course_dashboard_permission_names() -> Vec<String> {
    [
        Permissions::MANAGE_COURSE_SETTINGS,
        Permissions::CREATE_CONTENT,
        Permissions::MODIFY_CONTENT,
        Permissions::APPROVE_COURSE_CONTENT,
        Permissions::MANAGE_COURSE_ENROLLMENTS,
        Permissions::APPROVE_COURSE_JOIN_REQUESTS,
        Permissions::VIEW_COURSE_REWARD_STATUS,
        Permissions::APPROVE_STUDENT_REWARD_CANDIDATE,
        Permissions::MANAGE_COURSE_REWARD_RULES,
    ]
    .into_iter()
    .map(|permission| permission.to_string())
    .collect()
}

pub async fn build_teacher_course_permissions(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    course_id: i32,
) -> Result<TeacherCoursePermissionSummaryOutput, TeacherCourseDashboardError> {
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

    Ok(TeacherCoursePermissionSummaryOutput {
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
    permission_checks::can(
        conn,
        AccessActor::user(actor_user_id),
        AccessAction::permission(permission.to_string()),
        AccessScope::course(course_id),
    )
    .await
    .map_err(map_dashboard_error)
}

fn map_dashboard_error(error: diesel::result::Error) -> TeacherCourseDashboardError {
    match error {
        diesel::result::Error::NotFound => TeacherCourseDashboardError::NotFound,
        other => TeacherCourseDashboardError::Database(other.to_string()),
    }
}
