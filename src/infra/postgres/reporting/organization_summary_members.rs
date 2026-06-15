use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::reporting::organization_summary::OrganizationSummaryError;
use crate::db::schema::user_role_organization;
use crate::infra::postgres::reporting::organization_summary_mappers::map_diesel_error;

pub(super) async fn organization_member_user_ids(
    conn: &mut AsyncPgConnection,
    organization_id: i32,
) -> Result<Vec<Option<i32>>, OrganizationSummaryError> {
    user_role_organization::table
        .filter(user_role_organization::organization_id.eq(organization_id))
        .select(user_role_organization::user_id)
        .distinct()
        .load::<Option<i32>>(conn)
        .await
        .map_err(map_diesel_error)
}
