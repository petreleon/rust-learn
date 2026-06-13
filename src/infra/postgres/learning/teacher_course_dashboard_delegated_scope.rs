use chrono::Utc;
use diesel::prelude::*;
use diesel::BoolExpressionMethods;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::learning::teacher_course_dashboard::TeacherCourseDashboardError;
use crate::db::schema::delegated_permissions;
use crate::models::delegated_permission::{DELEGATED_SCOPE_COURSE, DELEGATED_SCOPE_ORGANIZATION};

pub async fn delegated_teacher_course_ids(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    permissions: &[String],
) -> Result<Vec<i32>, TeacherCourseDashboardError> {
    let now = Utc::now();
    let rows = delegated_permissions::table
        .filter(delegated_permissions::grantee_user_id.eq(actor_user_id))
        .filter(delegated_permissions::scope_type.eq(DELEGATED_SCOPE_COURSE))
        .filter(delegated_permissions::organization_id.is_null())
        .filter(delegated_permissions::permission.eq_any(permissions))
        .filter(delegated_permissions::revoked_at.is_null())
        .filter(
            delegated_permissions::expires_at
                .is_null()
                .or(delegated_permissions::expires_at.gt(now)),
        )
        .select(delegated_permissions::course_id)
        .load::<Option<i32>>(conn)
        .await
        .map_err(map_dashboard_error)?;

    Ok(rows.into_iter().flatten().collect())
}

pub async fn delegated_teacher_organization_ids(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    permissions: &[String],
) -> Result<Vec<i32>, TeacherCourseDashboardError> {
    let now = Utc::now();
    let rows = delegated_permissions::table
        .filter(delegated_permissions::grantee_user_id.eq(actor_user_id))
        .filter(delegated_permissions::scope_type.eq(DELEGATED_SCOPE_ORGANIZATION))
        .filter(delegated_permissions::course_id.is_null())
        .filter(delegated_permissions::permission.eq_any(permissions))
        .filter(delegated_permissions::revoked_at.is_null())
        .filter(
            delegated_permissions::expires_at
                .is_null()
                .or(delegated_permissions::expires_at.gt(now)),
        )
        .select(delegated_permissions::organization_id)
        .load::<Option<i32>>(conn)
        .await
        .map_err(map_dashboard_error)?;

    Ok(rows.into_iter().flatten().collect())
}

fn map_dashboard_error(error: diesel::result::Error) -> TeacherCourseDashboardError {
    match error {
        diesel::result::Error::NotFound => TeacherCourseDashboardError::NotFound,
        other => TeacherCourseDashboardError::Database(other.to_string()),
    }
}
