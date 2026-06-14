use crate::config::constants::permissions::Permissions;
use crate::db::schema::{
    delegated_permissions, role_permission_course, role_permission_organization,
    role_permission_platform, user_role_course, user_role_organization, user_role_platform,
};
use crate::domain::access_control::delegation::DELEGATED_SCOPE_PLATFORM;
use chrono::Utc;
use diesel::prelude::*;
use diesel::BoolExpressionMethods;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use std::collections::BTreeSet;

use super::errors::TeacherCourseDashboardError;
use super::teacher_delegated_scope::{
    courses_for_organizations, delegated_teacher_course_ids, delegated_teacher_organization_ids,
};

pub(super) enum TeacherCourseCandidateScope {
    All,
    CourseIds(Vec<i32>),
}

pub(super) async fn teacher_course_candidate_scope(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
) -> Result<TeacherCourseCandidateScope, TeacherCourseDashboardError> {
    let permission_names = teacher_course_dashboard_permission_names();
    if has_platform_teacher_course_scope(conn, actor_user_id, &permission_names).await? {
        return Ok(TeacherCourseCandidateScope::All);
    }

    let mut course_ids = BTreeSet::new();

    for course_id in direct_teacher_course_ids(conn, actor_user_id, &permission_names).await? {
        course_ids.insert(course_id);
    }
    for course_id in delegated_teacher_course_ids(conn, actor_user_id, &permission_names).await? {
        course_ids.insert(course_id);
    }

    let mut organization_ids = BTreeSet::new();
    for organization_id in
        direct_teacher_organization_ids(conn, actor_user_id, &permission_names).await?
    {
        organization_ids.insert(organization_id);
    }
    for organization_id in
        delegated_teacher_organization_ids(conn, actor_user_id, &permission_names).await?
    {
        organization_ids.insert(organization_id);
    }

    let organization_ids: Vec<i32> = organization_ids.into_iter().collect();
    for course_id in courses_for_organizations(conn, &organization_ids).await? {
        course_ids.insert(course_id);
    }

    Ok(TeacherCourseCandidateScope::CourseIds(
        course_ids.into_iter().collect(),
    ))
}

pub(super) fn teacher_course_dashboard_permission_names() -> Vec<String> {
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

async fn has_platform_teacher_course_scope(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    permission_names: &[String],
) -> Result<bool, TeacherCourseDashboardError> {
    let has_direct_platform_scope = diesel::select(diesel::dsl::exists(
        user_role_platform::table
            .inner_join(role_permission_platform::table.on(
                user_role_platform::platform_role_id.eq(role_permission_platform::platform_role_id),
            ))
            .filter(user_role_platform::user_id.eq(actor_user_id))
            .filter(role_permission_platform::permission.eq_any(permission_names)),
    ))
    .get_result::<bool>(conn)
    .await?;
    if has_direct_platform_scope {
        return Ok(true);
    }

    let now = Utc::now();
    diesel::select(diesel::dsl::exists(
        delegated_permissions::table
            .filter(delegated_permissions::grantee_user_id.eq(actor_user_id))
            .filter(delegated_permissions::scope_type.eq(DELEGATED_SCOPE_PLATFORM))
            .filter(delegated_permissions::organization_id.is_null())
            .filter(delegated_permissions::course_id.is_null())
            .filter(delegated_permissions::permission.eq_any(permission_names))
            .filter(delegated_permissions::revoked_at.is_null())
            .filter(
                delegated_permissions::expires_at
                    .is_null()
                    .or(delegated_permissions::expires_at.gt(now)),
            ),
    ))
    .get_result::<bool>(conn)
    .await
    .map_err(TeacherCourseDashboardError::from)
}

async fn direct_teacher_course_ids(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    permission_names: &[String],
) -> Result<Vec<i32>, TeacherCourseDashboardError> {
    let rows = user_role_course::table
        .inner_join(
            role_permission_course::table
                .on(user_role_course::course_role_id.eq(role_permission_course::course_role_id)),
        )
        .filter(user_role_course::user_id.eq(actor_user_id))
        .filter(role_permission_course::permission.eq_any(permission_names))
        .select(user_role_course::course_id)
        .load::<Option<i32>>(conn)
        .await?;

    Ok(rows.into_iter().flatten().collect())
}

async fn direct_teacher_organization_ids(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    permission_names: &[String],
) -> Result<Vec<i32>, TeacherCourseDashboardError> {
    let rows = user_role_organization::table
        .inner_join(
            role_permission_organization::table.on(user_role_organization::organization_role_id
                .eq(role_permission_organization::organization_role_id)),
        )
        .filter(user_role_organization::user_id.eq(actor_user_id))
        .filter(role_permission_organization::permission.eq_any(permission_names))
        .select(user_role_organization::organization_id)
        .load::<Option<i32>>(conn)
        .await?;

    Ok(rows.into_iter().flatten().collect())
}
