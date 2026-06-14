use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::reporting::organization_summary::OrganizationSummaryError;
use crate::db::schema::user_role_course;
use crate::infra::postgres::reporting::organization_summary_mappers::map_diesel_error;

pub(super) async fn course_role_assignment_count(
    conn: &mut AsyncPgConnection,
    course_ids: &[i32],
) -> Result<i64, OrganizationSummaryError> {
    if course_ids.is_empty() {
        return Ok(0);
    }
    user_role_course::table
        .filter(user_role_course::course_id.eq_any(course_ids))
        .count()
        .get_result(conn)
        .await
        .map_err(map_diesel_error)
}
