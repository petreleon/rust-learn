use std::collections::BTreeSet;

use chrono::Utc;
use diesel::dsl::{exists, select};
use diesel::prelude::*;
use diesel::BoolExpressionMethods;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::learning::teacher_course_dashboard::TeacherCourseDashboardError;
use crate::db::schema::{
    courses_organizations, delegated_permissions, role_permission_course,
    role_permission_organization, role_permission_platform, user_role_course,
    user_role_organization, user_role_platform,
};
use crate::infra::postgres::learning::{
    teacher_course_dashboard_delegated_scope, teacher_course_dashboard_permissions,
};
use crate::models::delegated_permission::DELEGATED_SCOPE_PLATFORM;

pub enum TeacherCourseCandidateScope {
    All,
    CourseIds(Vec<i32>),
}

pub async fn teacher_course_candidate_scope(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
) -> Result<TeacherCourseCandidateScope, TeacherCourseDashboardError> {
    let permissions =
        teacher_course_dashboard_permissions::teacher_course_dashboard_permission_names();
    if has_platform_teacher_course_scope(conn, actor_user_id, &permissions).await? {
        return Ok(TeacherCourseCandidateScope::All);
    }

    let mut course_ids = BTreeSet::new();
    for course_id in direct_teacher_course_ids(conn, actor_user_id, &permissions).await? {
        course_ids.insert(course_id);
    }
    for course_id in teacher_course_dashboard_delegated_scope::delegated_teacher_course_ids(
        conn,
        actor_user_id,
        &permissions,
    )
    .await?
    {
        course_ids.insert(course_id);
    }

    let mut organization_ids = BTreeSet::new();
    for organization_id in
        direct_teacher_organization_ids(conn, actor_user_id, &permissions).await?
    {
        organization_ids.insert(organization_id);
    }
    for organization_id in
        teacher_course_dashboard_delegated_scope::delegated_teacher_organization_ids(
            conn,
            actor_user_id,
            &permissions,
        )
        .await?
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

async fn has_platform_teacher_course_scope(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    permissions: &[String],
) -> Result<bool, TeacherCourseDashboardError> {
    let direct = select(exists(
        user_role_platform::table
            .inner_join(role_permission_platform::table.on(
                user_role_platform::platform_role_id.eq(role_permission_platform::platform_role_id),
            ))
            .filter(user_role_platform::user_id.eq(actor_user_id))
            .filter(role_permission_platform::permission.eq_any(permissions)),
    ))
    .get_result::<bool>(conn)
    .await
    .map_err(map_dashboard_error)?;
    if direct {
        return Ok(true);
    }

    let now = Utc::now();
    select(exists(
        delegated_permissions::table
            .filter(delegated_permissions::grantee_user_id.eq(actor_user_id))
            .filter(delegated_permissions::scope_type.eq(DELEGATED_SCOPE_PLATFORM))
            .filter(delegated_permissions::organization_id.is_null())
            .filter(delegated_permissions::course_id.is_null())
            .filter(delegated_permissions::permission.eq_any(permissions))
            .filter(delegated_permissions::revoked_at.is_null())
            .filter(
                delegated_permissions::expires_at
                    .is_null()
                    .or(delegated_permissions::expires_at.gt(now)),
            ),
    ))
    .get_result::<bool>(conn)
    .await
    .map_err(map_dashboard_error)
}

async fn direct_teacher_course_ids(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    permissions: &[String],
) -> Result<Vec<i32>, TeacherCourseDashboardError> {
    let rows = user_role_course::table
        .inner_join(
            role_permission_course::table
                .on(user_role_course::course_role_id.eq(role_permission_course::course_role_id)),
        )
        .filter(user_role_course::user_id.eq(actor_user_id))
        .filter(role_permission_course::permission.eq_any(permissions))
        .select(user_role_course::course_id)
        .load::<Option<i32>>(conn)
        .await
        .map_err(map_dashboard_error)?;

    Ok(rows.into_iter().flatten().collect())
}

async fn direct_teacher_organization_ids(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    permissions: &[String],
) -> Result<Vec<i32>, TeacherCourseDashboardError> {
    let rows = user_role_organization::table
        .inner_join(
            role_permission_organization::table.on(user_role_organization::organization_role_id
                .eq(role_permission_organization::organization_role_id)),
        )
        .filter(user_role_organization::user_id.eq(actor_user_id))
        .filter(role_permission_organization::permission.eq_any(permissions))
        .select(user_role_organization::organization_id)
        .load::<Option<i32>>(conn)
        .await
        .map_err(map_dashboard_error)?;

    Ok(rows.into_iter().flatten().collect())
}

async fn courses_for_organizations(
    conn: &mut AsyncPgConnection,
    organization_ids: &[i32],
) -> Result<Vec<i32>, TeacherCourseDashboardError> {
    if organization_ids.is_empty() {
        return Ok(Vec::new());
    }

    courses_organizations::table
        .filter(courses_organizations::organization_id.eq_any(organization_ids))
        .select(courses_organizations::course_id)
        .load::<i32>(conn)
        .await
        .map_err(map_dashboard_error)
}

fn map_dashboard_error(error: diesel::result::Error) -> TeacherCourseDashboardError {
    match error {
        diesel::result::Error::NotFound => TeacherCourseDashboardError::NotFound,
        other => TeacherCourseDashboardError::Database(other.to_string()),
    }
}
