use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::reporting::organization_summary::OrganizationSummaryError;
use crate::db::schema::courses_organizations;
use crate::infra::postgres::reporting::organization_summary_mappers::map_diesel_error;

pub(super) async fn organization_course_ids(
    conn: &mut AsyncPgConnection,
    organization_id: i32,
) -> Result<Vec<i32>, OrganizationSummaryError> {
    courses_organizations::table
        .filter(courses_organizations::organization_id.eq(organization_id))
        .select(courses_organizations::course_id)
        .load::<i32>(conn)
        .await
        .map_err(map_diesel_error)
}
